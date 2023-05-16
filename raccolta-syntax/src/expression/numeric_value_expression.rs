// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

/// ```text
/// <numeric value expression> ::=
///       <term>
///     | <numeric value expression> <plus sign> <term>
///     | <numeric value expression> <minus sign> <term>
///
/// <term> ::=
///       <factor>
///     | <term> <asterisk> <factor>
///     | <term> <solidus> <factor>
///
/// <factor> ::=
///     [ <sign> ] <numeric primary>
///
/// <numeric primary> ::=
///       <value expression primary>
///     | <numeric value function>
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum NumericValueExpression {
    SimpleU64(u64),
}
