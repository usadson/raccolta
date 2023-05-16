// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

/// ```text
/// <table reference> ::=
///       <table primary>
///     | <joined table>
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum TableReference {
    Primary(TablePrimary),
}

/// ```text
/// <table primary> ::=
///       <table or query name> [ [ AS ] <correlation name>
///     [ <left paren> <derived column list> <right paren> ] ]
///
///     | <derived table> [ AS ] <correlation name>
///       [ <left paren> <derived column list> <right paren> ]
///
///     | <lateral derived table> [ AS ] <correlation name>
///       [ <left paren> <derived column list> <right paren> ]
///
///     | <collection derived table> [ AS ] <correlation name>
///       [ <left paren> <derived column list> <right paren> ]
///
///     | <only spec>
///       [ [ AS ] <correlation name>
///         [ <left paren> <derived column list> <right paren> ] ]
///
///     | <left paren> <joined table> <right paren>
///
/// <only spec> ::=
///     ONLY <left paren> <table or query name> <right paren>
///
/// <lateral derived table> ::=
///     LATERAL <left paren> <query expression> <right paren>
///
/// <collection derived table> ::=
///       UNNEST <left paren> <collection value expression> <right paren>
///     [ WITH ORDINALITY ]
///
/// <derived table> ::= <table subquery>
/// <table or query name> ::=
///       <table name>
///     | <query name>
///
/// <derived column list> ::= <column name list>
///
/// <column name list> ::=
///     <column name> [ { <comma> <column name> }... ]
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct TablePrimary {
    pub kind: TablePrimaryKind,
    pub correlation_name: Option<String>,
    // pub derived_column_list: Vec<String>,
}

/// ```text
/// <table primary> ::=
///       <table or query name> [ [ AS ] <correlation name>
///     [ <left paren> <derived column list> <right paren> ] ]
///
///     | <derived table> [ AS ] <correlation name>
///       [ <left paren> <derived column list> <right paren> ]
///
///     | <lateral derived table> [ AS ] <correlation name>
///       [ <left paren> <derived column list> <right paren> ]
///
///     | <collection derived table> [ AS ] <correlation name>
///       [ <left paren> <derived column list> <right paren> ]
///
///     | <only spec>
///       [ [ AS ] <correlation name>
///         [ <left paren> <derived column list> <right paren> ] ]
///
///     | <left paren> <joined table> <right paren>
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum TablePrimaryKind {
    /// The name of a table or query.
    /// ```text
    /// <table or query name> ::= <identifier>
    /// ```
    TableOrQueryName(String),
}
