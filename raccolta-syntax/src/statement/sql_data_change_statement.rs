// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use super::InsertStatement;

/// ```text
/// <SQL data change statement> ::=
///       <delete statement: positioned>
///     | <delete statement: searched>
///     | <insert statement>
///     | <update statement: positioned>
///     | <update statement: searched>
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum SqlDataChangeStatement {
    /// The `INSERT INTO` statement, i.e. `<insert statement>`:
    ///
    /// ```text
    /// <insert statement> ::=
    ///     INSERT INTO <insertion target>
    ///     <insert columns and source>
    /// ```
    Insert(InsertStatement),
}
