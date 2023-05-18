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
    }
};

use crate::{
    EngineResult,
    table::EngineTable,
};

/// Execute a `SELECT` statement.
pub fn execute(statement: QuerySpecification, table: Arc<RwLock<EngineTable>>) -> EngineResult {
    match &statement.select_list {
        SelectList::Asterisk => execute_select_return_all(table),
        SelectList::Sublist(sublist) => execute_select_sublist(table, sublist),
    }
}

fn execute_select_sublist(table: Arc<RwLock<EngineTable>>, sublist: &[SelectSublist]) -> EngineResult {
    _ = table;
    _ = sublist;
    todo!();
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
