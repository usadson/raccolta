// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use raccolta_syntax::expression::{
    data_type::DataType,
    NumericValueExpression,
    row_value_constructor::ContextuallyTypedRowValueConstructorElement,
    ValueExpression,
};

use crate::{EngineResult, EngineRowColumnValue};

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
    pub name: String,
    pub columns: Vec<EngineColumn>,
}
