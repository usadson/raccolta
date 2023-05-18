// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::sync::{
    Arc,
    RwLock,
};

use raccolta_syntax::expression::{
    data_type::DataType,
    NumericValueExpression,
    row_value_constructor::ContextuallyTypedRowValueConstructorElement,
    ValueExpression,
};

use crate::{EngineResult, EngineRow, EngineRowColumnValue};

#[derive(Debug)]
pub struct EngineColumn {
    pub descriptor: EngineColumnDescriptor,
    pub values: EngineColumnContainer,
}

impl EngineColumn {
    pub fn append(&mut self, value: ContextuallyTypedRowValueConstructorElement) -> Result<(), EngineResult> {
        match value {
            ContextuallyTypedRowValueConstructorElement::ValueExpression(expression) => match expression {
                ValueExpression::Numeric(numeric_expression) => match numeric_expression {
                    NumericValueExpression::SimpleU64(value) => {
                        match &mut self.values {
                            EngineColumnContainer::Integers(vec) => {
                                vec.push(value as i32);
                                Ok(())
                            }
                        }
                    }
                }
                _ => todo!()
            }
        }
    }
}

#[derive(Debug)]
pub enum EngineColumnContainer {
    Integers(Vec<i32>),
}

impl EngineColumnContainer {
    pub fn len(&self) -> usize {
        match self {
            Self::Integers(vec) => vec.len(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct EngineColumnDescriptor {
    pub name: String,
    pub data_type: DataType,
}

/// A table as represented in the engine.
#[derive(Debug)]
pub struct EngineTable {
    pub name: Arc<str>,
    pub columns: Vec<EngineColumn>,
}

impl EngineTable {
    pub fn iter(instance: Arc<RwLock<Self>>) -> EngineTableColumnIterator {
        EngineTableColumnIterator {
            instance,
            idx: 0,
        }
    }
}

pub struct EngineTableColumnIterator {
    instance: Arc<RwLock<EngineTable>>,
    idx: usize,
}

impl Iterator for EngineTableColumnIterator {
    type Item = EngineRow;

    fn next(&mut self) -> Option<Self::Item> {
        let table = self.instance.read().unwrap();
        if table.columns.is_empty() {
            return None;
        }

        let idx = self.idx;
        if table.columns[0].values.len() <= idx {
            return None;
        }

        self.idx += 1;

        Some(EngineRow {
            values: table.columns
                .iter()
                .map(|column| {
                    match &column.values {
                        EngineColumnContainer::Integers(vec) => EngineRowColumnValue::I32(vec[idx]),
                    }
                })
                .collect()
        })
    }
}
