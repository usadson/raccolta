// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use super::SqlDataStatement;

#[derive(Clone, Debug, PartialEq)]
pub enum SqlExecutableStatement {
    SqlDataStatement(SqlDataStatement),
}
