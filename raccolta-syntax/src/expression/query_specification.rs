// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::set_function::SetQuantifier;

use super::{TableExpression, ValueExpression};

/// ```text
/// <derived column> ::=
///     <value expression> [ <as clause> ]
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct DerivedColumn {
    pub value_expression: ValueExpression,

    /// The `<as clause> <column name>`
    pub alias: Option<String>,
}

/// ```text
/// <query specification> ::=
///     SELECT [ <set quantifier> ] <select list>
///     <table expression>
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct QuerySpecification {
    pub set_quantifier: SetQuantifier,
    pub select_list: SelectList,
    pub table_expression: Option<TableExpression>,
}

/// ```text
/// <select list> ::=
///       <asterisk>
///     | <select sublist> [ { <comma> <select sublist> }... ]
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum SelectList {
    /// '*' or wildcard: select all rows from all referenced tables
    /// (in FROM-clause, subclause and JOIN-clauses).
    Asterisk,
    Sublist(Vec<SelectSublist>),
}

/// ```text
/// <select sublist> ::=
///       <derived column>
///     | <qualified asterisk>
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum SelectSublist {
    DerivedColumn(DerivedColumn),
}
