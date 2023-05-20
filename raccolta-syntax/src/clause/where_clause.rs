// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::expression::ValueExpression;

#[derive(Clone, Debug, PartialEq)]
pub struct WhereClause {
    pub search_condition: ValueExpression,
}
