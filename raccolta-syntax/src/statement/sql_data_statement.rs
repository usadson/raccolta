// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::expression::QueryExpression;

use super::{
    SelectStatementSingleRow,
    SqlDataChangeStatement,
};

/// ```text
/// <SQL data statement> ::=
///       <open statement>
///     | <fetch statement>
///     | <close statement>
///     | <select statement: single row>
///     | <free locator statement>
///     | <hold locator statement>
///     | <SQL data change statement>
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum SqlDataStatement {
    /// `<SQL data change statement>`
    ChangeStatement(SqlDataChangeStatement),
    SelectStatement(QueryExpression),
    SelectStatementSingleRow(SelectStatementSingleRow),
}
