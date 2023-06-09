// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

//! This module contains the types that manage the table and underlying columns.

use std::sync::{
    Arc,
    RwLock, RwLockReadGuard,
};

use bitvec::vec::BitVec;
use raccolta_syntax::expression::{
    data_type::DataType,
    NumericValueExpression,
    row_value_constructor::ContextuallyTypedRowValueConstructorElement,
    string_value_expression::StringValueExpression,
    ValueExpression, BooleanExpression,
};

use crate::{
    EngineMessage,
    EngineResult,
    EngineRow,
    EngineRowColumnValue,
};

#[derive(Debug)]
pub struct EngineColumn {
    pub descriptor: EngineColumnDescriptor,
    pub values: EngineColumnContainer,
}

impl EngineColumn {
    pub fn append(&mut self, value: ContextuallyTypedRowValueConstructorElement) -> Result<(), EngineResult> {
        let internal_coercion_error = |column, value| {
            Err(EngineResult::with_messages(vec![
                EngineMessage::Error(format!(
                    "Internal Error: non-coerced type {value:#?} as candidate for appending to column {column:?}",
                ).into())
            ]))
        };

        match value {
            ContextuallyTypedRowValueConstructorElement::ValueExpression(expression) => match expression {
                ValueExpression::Boolean(BooleanExpression::Literal(value)) => match &mut self.values {
                    EngineColumnContainer::Booleans(vec) => {
                        vec.push(value);
                        Ok(())
                    }
                    EngineColumnContainer::Integers(vec) => {
                        vec.push(if value { 1 } else { 0 });
                        Ok(())
                    }
                    _ => internal_coercion_error(&self, ContextuallyTypedRowValueConstructorElement::ValueExpression(
                        ValueExpression::Boolean(BooleanExpression::Literal(value))
                    ))
                }

                ValueExpression::Numeric(numeric_expression) => match numeric_expression {
                    NumericValueExpression::SimpleU64(number) => {
                        match &mut self.values {
                            EngineColumnContainer::Integers(vec) => {
                                vec.push(number as i32);
                                Ok(())
                            }
                            _ => internal_coercion_error(&self, ContextuallyTypedRowValueConstructorElement::ValueExpression(
                                ValueExpression::Numeric(NumericValueExpression::SimpleU64(number)))
                            )
                        }
                    }
                }

                ValueExpression::StringValueExpression(string_expression) => match string_expression {
                    StringValueExpression::Literal(mut literal) => {
                        match &mut self.values {
                            EngineColumnContainer::StringsVarying { maximum_length, values } => {
                                literal.truncate(*maximum_length);
                                values.push(literal);
                                Ok(())
                            }
                            _ => internal_coercion_error(&self, ContextuallyTypedRowValueConstructorElement::ValueExpression(
                                ValueExpression::StringValueExpression(StringValueExpression::Literal(literal)))
                            )
                        }
                    }
                }

                _ => internal_coercion_error(&self, ContextuallyTypedRowValueConstructorElement::ValueExpression(expression))
            }
        }
    }
}

#[derive(Debug)]
pub enum EngineColumnContainer {
    Booleans(BitVec),

    Integers(Vec<i32>),

    StringsVarying {
        values: Vec<String>,
        maximum_length: usize,
    },
}

impl EngineColumnContainer {
    pub fn len(&self) -> usize {
        match self {
            Self::Booleans(vec) => vec.len(),
            Self::Integers(vec) => vec.len(),
            Self::StringsVarying{ values, .. } => values.len(),
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
            selected_column_indices: None
        }
    }

    pub fn iter_with(instance: Arc<RwLock<Self>>, selected_column_indices: Vec<usize>) -> EngineTableColumnIterator {
        EngineTableColumnIterator {
            instance,
            idx: 0,
            selected_column_indices: Some(selected_column_indices)
        }
    }
}

pub struct EngineTableColumnIterator {
    instance: Arc<RwLock<EngineTable>>,
    idx: usize,
    selected_column_indices: Option<Vec<usize>>,
}

impl EngineTableColumnIterator {
    fn next_with_indices<'lock>(
        &self,
        table: RwLockReadGuard<'lock, EngineTable>,
        selected_column_indices: &[usize],
        row_index: usize,
    ) -> EngineRow {
        EngineRow {
            values: selected_column_indices.into_iter()
                .map(|column_index| {
                    let column = &table.columns[*column_index];

                    match &column.values {
                        EngineColumnContainer::Booleans(vec) =>
                            EngineRowColumnValue::Bool(vec[row_index]),
                        EngineColumnContainer::Integers(vec) =>
                            EngineRowColumnValue::I32(vec[row_index]),
                        EngineColumnContainer::StringsVarying { values, .. } =>
                            EngineRowColumnValue::String(values[row_index].clone())
                    }
                })
                .collect()
        }
    }
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

        if let Some(selected_column_indices) = &self.selected_column_indices {
            return Some(self.next_with_indices(table, selected_column_indices, idx));
        }

        Some(EngineRow {
            values: table.columns
                .iter()
                .map(|column| {
                    match &column.values {
                        EngineColumnContainer::Booleans(vec) => EngineRowColumnValue::Bool(vec[idx]),
                        EngineColumnContainer::Integers(vec) => EngineRowColumnValue::I32(vec[idx]),
                        EngineColumnContainer::StringsVarying { values, .. } => EngineRowColumnValue::String(values[idx].clone()),
                    }
                })
                .collect()
        })
    }
}
