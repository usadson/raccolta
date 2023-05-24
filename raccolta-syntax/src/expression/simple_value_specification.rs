// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

/// ```text
/// simple_value_specification ::=
///       literal
///     | host_parameter_name
///     | SQL_parameter_reference
///     | embedded_variable_name
/// ```
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SimpleValueSpecification {
    LiteralUnsigned(u64),
}
