// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::set_function::SetQuantifier;

/// The `<select statement: single row>` statement.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SelectStatementSingleRow {
    pub set_quantifier: SetQuantifier,
}
