// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

//! ISO/IEC 9075-2:1999 - Framework - 7.2 <row value expression>
//!
//! # Definitions
//! ```text
//! <row value expression> ::=
//!       <row value special case>
//!     | <row value constructor>
//!
//! <contextually typed row value expression> ::=
//!       <row value special case>
//!     | <contextually typed row value constructor>
//!
//! <row value special case> ::=
//!       <value specification>
//!     | <value expression>
//! ```

use crate::expression::row_value_constructor::ContextuallyTypedRowValueConstructor;

/// ```text
/// <contextually typed row value expression> ::=
///       <row value special case>
///     | <contextually typed row value constructor>
///
/// <row value special case> ::=
///       <value specification>
///     | <value expression>
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum ContextuallyTypedRowValueExpression {
    ContextuallyTypedRowValueConstructor(ContextuallyTypedRowValueConstructor),
}
