// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

//! This module contains the logic for executing `SELECT` statements.

use std::sync::{
    Arc,
    RwLock
};

use raccolta_syntax::{expression::{
    query_specification::{
        QuerySpecification,
        SelectList, SelectSublist
    }, ColumnReference, ValueExpression
}, clause::order_by_clause::{OrderByClause, OrderingSpecification}};

use crate::{
    EngineMessage,
    EngineResult,
    sorting::{EngineRowSortIteratorExtensionTrait, EngineSortingMethod, EngineSortingElement},
    table::EngineTable,
};

/// Execute a `SELECT` statement.
pub fn execute(
    statement: QuerySpecification,
    table: Arc<RwLock<EngineTable>>,
    order_by_clause: Option<OrderByClause>,
) -> EngineResult {
    match &statement.select_list {
        SelectList::Asterisk => execute_select_return_all(table, order_by_clause),
        SelectList::Sublist(sublist) => execute_select_sublist(table, order_by_clause, sublist),
    }
}

fn execute_select_sublist(
    table_ptr: Arc<RwLock<EngineTable>>,
    order_by_clause: Option<OrderByClause>,
    sublist: &[SelectSublist],
) -> EngineResult {
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
                                None => return EngineResult::with_messages(vec![
                                    EngineMessage::Error(format!("failed to find column: \"{}\"", chain.last().unwrap()).into())
                                ])
                            }

                            continue;
                        }
                    }
                    _ => ()
                }
            }
        }

        return EngineResult::with_messages(vec![
            EngineMessage::Error(format!("column not resolvable: {select_element:?}").into())
        ]);
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

    let sorting_method = match resolve_sorting_method(&column_names, order_by_clause) {
        Ok(sorting_method) => sorting_method,
        Err(engine_result) => return engine_result,
    };

    drop(table);

    EngineResult {
        messages: Vec::new(),
        column_names,
        row_count,
        row_iterator: EngineTable::iter_with(table_ptr, column_indices)
            .apply_order_by(sorting_method)
    }
}

fn execute_select_return_all(
    table_ptr: Arc<RwLock<EngineTable>>,
    order_by_clause: Option<OrderByClause>,
) -> EngineResult {
    let table = table_ptr.as_ref().read().unwrap();

    let column_names = table.columns.iter()
        .map(|column| column.descriptor.name.clone())
        .collect();

    let row_count = table.columns[0].values.len();

    // Ha ha, this isn't what it seams like :^)
    drop(table);

    let sorting_method = match resolve_sorting_method(
        &column_names,
        order_by_clause
    ) {
        Ok(sorting_method) => sorting_method,
        Err(engine_result) => return engine_result,
    };

    EngineResult {
        messages: Vec::new(),
        column_names,
        row_count,
        row_iterator: EngineTable::iter(table_ptr)
            .apply_order_by(sorting_method)
    }
}

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
