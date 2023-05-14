// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use super::{
    SqlDataStatement,
    SqlSchemaStatement,
};

#[derive(Clone, Debug, PartialEq)]
pub enum SqlExecutableStatement {
    Schema(SqlSchemaStatement),
    SqlDataStatement(SqlDataStatement),
}
