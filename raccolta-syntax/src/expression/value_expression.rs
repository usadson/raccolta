// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::set_function::SetFunctionSpecification;

use super::{
    ColumnReference,
    NumericValueExpression
};

/// ```text
/// <value expression> ::=
///       <numeric value expression>
///     | <string value expression>
///     | <datetime value expression>
///     | <interval value expression>
///     | <boolean value expression>
///     | <user-defined type value expression>
///     | <row value expression>
///     | <reference value expression>
///     | <collection value expression>
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum ValueExpression {
    /// `<numeric value expression>`
    Numeric(NumericValueExpression),

    ColumnReference(ColumnReference),

    SetFunctionSpecification(SetFunctionSpecification),
}
