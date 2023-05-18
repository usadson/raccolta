// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

/// ```text
/// <set function specification> ::=
///       COUNT <left paren> <asterisk> <right paren>
///     | <general set function>
///     | <grouping operation>
///
/// <general set function> ::=
///     <set function type>
///     <left paren> [ <set quantifier> ] <value expression> <right paren>
///
/// <set function type> ::= <computational operation>
///
/// <computational operation> ::=
///       AVG | MAX | MIN | SUM
///     | EVERY | ANY | SOME
///     | COUNT
///
/// <grouping operation> ::=
///     GROUPING <left paren> <column reference> <right paren>
/// ```
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SetFunctionSpecification {
    /// `COUNT(*)` i.e. `COUNT <left paren> <asterisk> <right paren>`
    Count,
}

/// The `<set quantifier>` specifies a quantification method of e.g. an
/// `SELECT`-query. This can be either **`DISTINCT`** or **`ALL`**.
///
/// **`ALL`** is implied if no `<set quantifier>` is provided.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum SetQuantifier {
    /// The **`ALL`** quantifier doesn't process the rows in the query result,
    /// and just returns all the rows, even if there might be redundant
    /// duplicate rows (as is removed by the **`DISTINCT`** quantifier).
    ///
    /// This is the default implicit value, but can be explicitly mentioned via
    /// the **`ALL`** keyword.
    #[default]
    All,

    /// The distinct quantifier removes redundant duplicate rows in the query
    /// result.
    Distinct,
}
