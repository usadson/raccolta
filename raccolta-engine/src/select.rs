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
    EngineRow,
    EngineRowColumnValue,
    table::{
        EngineColumnContainer,
        EngineTable,
    },
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
    assert_eq!(table.columns.len(), 1);

    EngineResult {
        messages: Vec::new(),
        column_names: table.columns.iter()
            .map(|column| column.descriptor.name.clone())
            .collect(),
        row_count: table.columns[0].values.len(),
        row_iterator: Box::new(match &table.columns[0].values {
            EngineColumnContainer::Integers(vec) => vec.clone()
                .into_iter()
                .map(|value| EngineRowColumnValue::I32(value))
                .map(|value| EngineRow { values: vec![value] })
        })
    }
}
