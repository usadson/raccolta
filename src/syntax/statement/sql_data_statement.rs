// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::syntax::expression::QueryExpression;

use super::SelectStatementSingleRow;

#[derive(Clone, Debug, PartialEq)]
pub enum SqlDataStatement {
    SelectStatement(QueryExpression),
    SelectStatementSingleRow(SelectStatementSingleRow),
}
