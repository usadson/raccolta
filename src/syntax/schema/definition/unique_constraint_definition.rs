// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

/// ```text
/// <unique specification> ::=
///       UNIQUE
///     | PRIMARY KEY
/// ```
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum UniqueSpecification {
    Unique,
    PrimaryKey,
}
