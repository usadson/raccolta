// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::set_function::SetFunctionSpecification;

use super::{
    BooleanExpression,
    ColumnReference,
    NumericValueExpression,
    string_value_expression::StringValueExpression,
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
    Boolean(BooleanExpression),

    ColumnReference(ColumnReference),

    /// `<numeric value expression>`
    Numeric(NumericValueExpression),

    SetFunctionSpecification(SetFunctionSpecification),

    StringValueExpression(StringValueExpression),
}
