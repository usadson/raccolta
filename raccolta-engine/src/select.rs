// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

//! This module contains the logic for executing `SELECT` statements.

use std::sync::{
    Arc,
    RwLock
};

use raccolta_syntax::{
    expression::{
        ColumnReference,
        query_specification::{
            QuerySpecification,
            SelectList,
            SelectSublist,
        },
        ValueExpression, SimpleValueSpecification,
    },
    clause::{
        fetch_first_clause::FetchFirstClause,
        order_by_clause::{
            OrderByClause,
            OrderingSpecification,
        },
    },
};

use crate::{
    EngineMessage,
    EngineResult,
    sorting::{
        EngineRowSortIteratorExtensionTrait,
        EngineSortingElement,
        EngineSortingMethod,
    },
    table::{
        EngineTable,
        EngineTableColumnIterator,
    },
};

struct SelectionPhaseResult {
    row_count: usize,
    column_names: Vec<String>,
    row_iterator: EngineTableColumnIterator,
}

/// Execute a `SELECT` statement.
pub fn execute(
    statement: QuerySpecification,
    table: Arc<RwLock<EngineTable>>,
    order_by_clause: Option<OrderByClause>,
    fetch_first_clause: Option<FetchFirstClause>,
) -> EngineResult {
    match execute_inner(statement, table, order_by_clause, fetch_first_clause) {
        Ok(result) => result,
        Err(result) => result,
    }
}

/// The inner function of [`execute`], which returns an [`Result`] for easy
/// error propagation.
fn execute_inner(
    statement: QuerySpecification,
    table: Arc<RwLock<EngineTable>>,
    order_by_clause: Option<OrderByClause>,
    fetch_first_clause: Option<FetchFirstClause>,
) -> Result<EngineResult, EngineResult> {
    let selection_phase = match &statement.select_list {
        SelectList::Asterisk => execute_select_return_all(table)?,
        SelectList::Sublist(sublist) => execute_select_sublist(table, sublist)?,
    };

    let sorting_method = resolve_sorting_method(&selection_phase.column_names, order_by_clause)?;

    let mut row_iterator = selection_phase.row_iterator
        .apply_order_by(sorting_method);

    let mut row_count = selection_phase.row_count;

    if let Some(fetch_first_clause) = fetch_first_clause {
        let max = match fetch_first_clause.quantity.value {
            SimpleValueSpecification::LiteralUnsigned(value) => value as _,
        };

        if max < selection_phase.row_count {
            row_iterator = Box::new(row_iterator.take(max));
            row_count = max;
        }
    }

    Ok(EngineResult {
        messages: Vec::new(),
        column_names: selection_phase.column_names,
        row_count,
        row_iterator,
    })
}

/// Get a selected list of columns from the table. This is different from
/// [`execute_select_return_all`], which returns an iterator with all columns.
///
/// # Example
/// ```sql
/// SELECT column_name, other_column_name
/// FROM table_name
/// ```
fn execute_select_sublist(
    table_ptr: Arc<RwLock<EngineTable>>,
    sublist: &[SelectSublist],
) -> Result<SelectionPhaseResult, EngineResult> {
    let table = table_ptr.as_ref().read().unwrap();

    let mut column_indices = Vec::with_capacity(sublist.len());
    for select_element in sublist {
        match select_element {
            SelectSublist::DerivedColumn(derived_column) => {
                match &derived_column.value_expression {
                    ValueExpression::ColumnReference(column) => {
                        if let ColumnReference::BasicIdentifierChain(chain) = column {
                            let column = table.columns.iter()
                                .enumerate()
                                .find(|column| {
                                    chain.last().unwrap().eq_ignore_ascii_case(&column.1.descriptor.name)
                                });
                            match column {
                                Some(column) => column_indices.push(column.0),
                                None => return Err(EngineResult::with_messages(vec![
                                    EngineMessage::Error(format!("failed to find column: \"{}\"", chain.last().unwrap()).into())
                                ]))
                            }

                            continue;
                        }
                    }
                    _ => ()
                }
            }
        }

        return Err(EngineResult::with_messages(vec![
            EngineMessage::Error(format!("column not resolvable: {select_element:?}").into())
        ]));
    }

    debug_assert_eq!(column_indices.len(), sublist.len());

    let column_names = column_indices.iter()
        .enumerate()
        .map(|(sublist_index, column_index)| {
            match &sublist[sublist_index] {
                SelectSublist::DerivedColumn(derived) => {
                    if let Some(alias) = &derived.alias {
                        return alias.clone();
                    }
                }
            }

            table.columns[*column_index].descriptor.name.clone()
        })
        .collect();

    let row_count = table.columns[0].values.len();

    drop(table);

    Ok(SelectionPhaseResult {
        column_names,
        row_count,
        row_iterator: EngineTable::iter_with(table_ptr, column_indices)
    })
}

/// Get a all columns from the table. This is different from
/// [`execute_select_sublist`], which returns an iterator with the selected
/// columns.
///
/// # Example
/// ```sql
/// SELECT *
/// FROM table_name
/// ```
fn execute_select_return_all(
    table_ptr: Arc<RwLock<EngineTable>>,
) -> Result<SelectionPhaseResult, EngineResult> {
    let table = table_ptr.as_ref().read().unwrap();

    let column_names = table.columns.iter()
        .map(|column| column.descriptor.name.clone())
        .collect();

    let row_count = table.columns[0].values.len();

    // Ha ha, this isn't what it seams like :^)
    drop(table);

    Ok(SelectionPhaseResult {
        column_names,
        row_count,
        row_iterator: EngineTable::iter(table_ptr),
    })
}

/// Resolve a [`EngineSortingMethod`], which means that the [`OrderByClause`] is
/// translated to steps that the engine can use. These steps tell the engine in
/// which way and which order to sort the table in.
fn resolve_sorting_method(
    column_names: &Vec<String>,
    order_by_clause: Option<OrderByClause>
) -> Result<EngineSortingMethod, EngineResult> {
    let Some(order_by_clause) = order_by_clause else {
        return Ok(EngineSortingMethod::new());
    };

    let mut sorting_method = EngineSortingMethod::with_capacity(order_by_clause.sort_specification_list.len());

    for specification in order_by_clause.sort_specification_list {
        match specification.sort_key {
            ColumnReference::BasicIdentifierChain(chain) => {
                match column_names.iter()
                    .enumerate()
                    .find(|(_, name)| {
                        chain.last().unwrap().eq_ignore_ascii_case(name)
                    }) {
                    Some((column_index, _)) => sorting_method.push(EngineSortingElement{
                        column_index,
                        ordering_specification: specification.ordering_specification.unwrap_or(OrderingSpecification::Ascending)
                    }),

                    None => return Err(EngineResult::with_messages(vec![
                        EngineMessage::Error(format!(
                            "invalid column reference in ORDER BY: {}", chain.last().unwrap(),
                        ).into()),
                        EngineMessage::Hint("If you used an alias, did you forget to use the alias name instead?".into())
                    ]))
                }
            }
            _ => return Err(EngineResult::with_messages(vec![
                EngineMessage::Error(format!(
                    "failed to resolve column reference: {:#?}",
                    specification
                ).into())
            ])),
        }
    }

    Ok(sorting_method)
}
