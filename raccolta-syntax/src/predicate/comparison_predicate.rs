// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

//! SQL 1999 - 8.2 `<comparison predicate>`
//! Definition
//! ```text
//! <comparison predicate> ::=
//!     <row value expression> <comp op> <row value expression>
//!
//! <comp op> ::=
//!       <equals operator>
//!     | <not equals operator>
//!     | <less than operator>
//!     | <greater than operator>
//!     | <less than or equals operator>
//!     | <greater than or equals operator>
//! ```

use crate::expression::ValueExpression;

/// The operator which is used for comparison. These operators define how two
/// values should be compared, and are functionally equivalent in many
/// programming languages as:
/// 1. Less than `<`
/// 2. Less than or equal to `<=`
/// 3. Greater than `>`
/// 4. Greater than or equal to `>=`
/// 5. Equal to `==` \
///    **NOTE** that in SQL, `=` is used. `=` is used in other languages as the
///    assignment operator, but SQL used different semantics (notably **`SET`**
///    clauses) to accomplish this.
/// 6. Not equal to `!=` \
///    **NOTE** again that in *standard* SQL, `<>` is used.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ComparisonOperator {
    /// This is the `<` token, denoting that the left-hand-side is less than
    /// the right-hand-side.
    LessThan,

    /// This is the `<=` token, denoting that the left-hand-side is less than
    /// or equal to the right-hand-side.
    LessThanOrEqualTo,

    /// This is the `<` token, denoting that the left-hand-side is greater than
    /// the right-hand-side.
    GreaterThan,

    /// This is the `<=` token, denoting that the left-hand-side is greater than
    /// or equal to the right-hand-side.
    GreaterThanOrEqualTo,

    /// This is the `=` token, denoting that the left-hand-side is equal to
    /// the right-hand-side.
    EqualTo,

    /// This is the `<>` token, denoting that the left-hand-side is equal to
    /// the right-hand-side.
    NotEqualTo,
}

/// A predicate that compares two values using a specified operator.
#[derive(Clone, Debug, PartialEq)]
pub struct ComparisonPredicate {
    /// The operator which is used for comparison, which define how two values
    /// should be compared.
    pub operator: ComparisonOperator,

    /// The first argument in an comparison.
    ///
    /// For example:
    /// ```sql
    /// SELECT ...
    /// FROM ...
    /// WHERE left_hand_side > right_hand_side
    /// ```
    pub left_hand_side: ValueExpression,

    /// The second argument in an comparison.
    ///
    /// For example:
    /// ```sql
    /// SELECT ...
    /// FROM ...
    /// WHERE left_hand_side > right_hand_side
    /// ```
    pub right_hand_side: ValueExpression,
}
