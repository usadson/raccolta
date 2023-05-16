// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use super::SqlSchemaDefinitionStatement;

#[derive(Clone, Debug, PartialEq)]
pub enum SqlSchemaStatement {
    Definition(SqlSchemaDefinitionStatement),
}
