// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

/// ```text
/// <table name> ::=
///     <local or schema qualified name>
///
/// <local or schema qualified name> ::=
///     [ <local or schema qualifier> <period> ] <qualified identifier>
///
/// <local or schema qualifier> ::=
////      <schema name>
////    | MODULE
///
//// <qualified identifier> ::= <identifier>
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct TableName {
    /// `<qualified identifier>`
    table_qualifier: String,
}
