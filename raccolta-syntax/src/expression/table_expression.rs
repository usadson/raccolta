// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::clause::{
    FromClause,
    GroupByClause,
    HavingClause,
    WhereClause,
};

/// ```text
/// <table expression> ::=
///     <from clause>
///     [ <where clause> ]
///     [ <group by clause> ]
///     [ <having clause> ]
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct TableExpression {
    pub from_clause: FromClause,
    pub where_clause: Option<WhereClause>,
    pub group_by_clause: Option<GroupByClause>,
    pub having_clause: Option<HavingClause>,
}
