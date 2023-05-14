// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::syntax::schema::definition::TableDefinition;

#[derive(Clone, Debug, PartialEq)]
pub enum SqlSchemaDefinitionStatement {
    Table(TableDefinition),
}
