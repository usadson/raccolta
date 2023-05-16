// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

/// ```text
/// <character set specification> ::=
///       <standard character set name>
///     | <implementation-defined character set name>
///     | <user-defined character set name>
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct CharacterSetSpecification {
    // TODO schema name

    pub name: String,
    pub kind: CharacterSetSpecificationKind,
}

/// ```text
/// <standard character set name> ::= <character set name>
/// <implementation-defined character set name> ::= <character set name>
/// <user-defined character set name> ::= <character set name>
/// ```
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CharacterSetSpecificationKind {
    Standard,
    ImplementationDefined,
    UserDefined,
}
