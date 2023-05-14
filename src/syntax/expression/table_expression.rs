// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::syntax::clause::{
    FromClause,
    GroupByClause,
    HavingClause,
    WhereClause,
};

#[derive(Clone, Debug, PartialEq)]
pub struct TableExpression {
    pub from_clause: FromClause,
    pub where_clause: Option<WhereClause>,
    pub group_by_clause: Option<GroupByClause>,
    pub having_clause: Option<HavingClause>,
}