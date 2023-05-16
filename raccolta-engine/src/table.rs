// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use raccolta_syntax::expression::data_type::DataType;

#[derive(Debug)]
pub struct EngineColumn {
    pub descriptor: EngineColumnDescriptor,
    pub values: EngineColumnContainer,
}

#[derive(Debug)]
pub enum EngineColumnContainer {
    Integers(Vec<i32>),
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
