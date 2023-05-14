// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::syntax::expression::data_type::DataType;

use super::UniqueSpecification;

/// ```text
/// <column constraint definition> ::=
///     [ <constraint name definition> ]
///     <column constraint> [ <constraint characteristics> ]
///
/// <column constraint> ::=
///       NOT NULL
///     | <unique specification>
///     | <references specification>
///     | <check constraint definition>
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum ColumnConstraintDefinition {
    NotNull,
    UniqueSpecification(UniqueSpecification),
}

/// ```text
/// <column definition> ::=
///       <column name>
///     { <data type> | <domain name> }
///     [ <reference scope check> ]
///     [ <default clause> ]
///     [ <column constraint definition>... ]
///     [ <collate clause> ]
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct ColumnDefinition {
    column_name: String,
    data_type: DataType,
    column_constraint_definitions: Vec<ColumnConstraintDefinition>,
}

/// ```text
// <reference scope check> ::=
//      REFERENCES ARE [ NOT ] CHECKED
//      [ ON DELETE <reference scope check action> ]
// ```

/// ```text
/// <table definition> ::=
///     CREATE [ <table scope> ] TABLE <table name>
///       <table contents source>
///       [ ON COMMIT <table commit action> ROWS ]
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct TableDefinition {
    table_name: String,

    elements: Vec<TableElement>,
}

/// ```text
/// <table element> ::=
///       <column definition>
///     | <table constraint definition>
///     | <like clause>
///     | <self-referencing column specification>
///     | <column options>
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum TableElement {
    ColumnDefinition(ColumnDefinition),
}
