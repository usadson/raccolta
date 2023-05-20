// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::clause::order_by_clause::OrderByClause;

use super::QuerySpecification;

/// ```text
/// <non-join query expression> ::=
///     <non-join query term>
///     | <query expression body> UNION [ ALL | DISTINCT ]
///       [ <corresponding spec> ] <query term>
///     | <query expression body> EXCEPT [ ALL | DISTINCT ]
///       [ <corresponding spec> ] <query term>
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum NonJoinQueryExpression {
    NonJoinQueryTerm(NonJoinQueryTerm),
}

/// ```text
/// <non-join query primary> ::=
///     <simple table>
///     | <left paren> <non-join query expression> <right paren>
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum NonJoinQueryPrimary {
    SimpleTable(SimpleTable),
}

/// ```text
/// <non-join query term> ::=
///     <non-join query primary>
///     | <query term> INTERSECT [ ALL | DISTINCT ]
///     [ <corresponding spec> ] <query primary>
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum NonJoinQueryTerm {
    NonJoinQueryPrimary(NonJoinQueryPrimary),
}

/// ```text
/// <query expression> ::=
///     [ <with clause> ] <query expression body>
/// ```
///
/// **TODO:** optional **`WITH`** clause
#[derive(Clone, Debug, PartialEq)]
pub struct QueryExpression {
    pub body: QueryExpressionBody,

    /// An optional **`ORDER BY`** clause.
    pub order_by: Option<OrderByClause>,
}

/// ```text
/// <query expression body> ::=
///       <non-join query expression>
///     | <joined table>
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum QueryExpressionBody {
    NonJoinQueryExpression(NonJoinQueryExpression),

    // Fast path
    SimpleTable(SimpleTable),
}

/// `<query primary>`
#[derive(Clone, Debug, PartialEq)]
pub enum QueryPrimary {
    NonJoinQueryPrimary(NonJoinQueryPrimary),
}

/// ```text
/// <simple table> ::=
///       <query specification>
///     | <table value constructor>
///     | <explicit table>
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum SimpleTable {
    QuerySpecification(QuerySpecification),
}
