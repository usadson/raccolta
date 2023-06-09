// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

/// ```text
/// <column reference> ::=
///       <basic identifier chain>
///     | MODULE <period> <qualified identifier> <period> <column name>
/// ```
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum ColumnReference {
    /// ```text
    /// <identifier chain> ::=
    ///     <identifier> [ { <period> <identifier> }... ]
    ///
    /// <basic identifier chain> ::=
    ///     <identifier chain>
    /// ```
    BasicIdentifierChain(Vec<String>),
}
