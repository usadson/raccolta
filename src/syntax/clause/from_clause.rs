// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::syntax::expression::TableReference;

/// ```text
/// <from clause> ::=
///     FROM <table reference list>
///
/// <table reference list> ::=
///     <table reference> [ { <comma> <table reference> }... ]
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct FromClause {
    pub table_references: Vec<TableReference>,
}
