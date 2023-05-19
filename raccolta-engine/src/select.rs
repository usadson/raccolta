// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::sync::{
    Arc,
    RwLock
};

use raccolta_syntax::expression::{
    query_specification::{
        QuerySpecification,
        SelectList, SelectSublist
    }, ColumnReference, ValueExpression
};

use crate::{
    EngineResult,
    table::EngineTable, EngineMessage,
};

/// Execute a `SELECT` statement.
pub fn execute(statement: QuerySpecification, table: Arc<RwLock<EngineTable>>) -> EngineResult {
    match &statement.select_list {
        SelectList::Asterisk => execute_select_return_all(table),
        SelectList::Sublist(sublist) => execute_select_sublist(table, sublist),
    }
}

fn execute_select_sublist(table_ptr: Arc<RwLock<EngineTable>>, sublist: &[SelectSublist]) -> EngineResult {
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


    drop(table);

    EngineResult {
        messages: Vec::new(),
        column_names,
        row_count,
        row_iterator: Box::new(EngineTable::iter_with(table_ptr, column_indices))
    }
}

fn execute_select_return_all(table_ptr: Arc<RwLock<EngineTable>>) -> EngineResult {
    let table = table_ptr.as_ref().read().unwrap();

    let column_names = table.columns.iter()
        .map(|column| column.descriptor.name.clone())
        .collect();

    let row_count = table.columns[0].values.len();

    // Ha ha, this isn't what it seams like :^)
    drop(table);

    EngineResult {
        messages: Vec::new(),
        column_names,
        row_count,
        row_iterator: Box::new(EngineTable::iter(table_ptr))
    }
}
