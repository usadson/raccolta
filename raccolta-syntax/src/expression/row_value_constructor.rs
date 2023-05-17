// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

//! ISO/IEC 9075-2:1999 - Framework - 7.1 <row value constructor>
//!
//! # Definitions
//! ```text
//! <row value constructor> ::=
//!       <row value constructor element>
//!     | [ ROW ] <left paren> <row value constructor element list> <right paren>
//!     | <row subquery>
//!
//! <row value constructor element list> ::=
//!     <row value constructor element>
//!     [ { <comma> <row value constructor element> }... ]
//!
//! <row value constructor element> ::=
//!     <value expression>
//!
//! <contextually typed row value constructor> ::=
//!       <contextually typed row value constructor element>
//!     | [ ROW ]
//!         <left paren>
//!             <contextually typed row value constructor element list>
//!         <right paren>
//!
//! <contextually typed row value constructor element list> ::=
//!     <contextually typed row value constructor element>
//!     [ { <comma> <contextually typed row value constructor element> }... ]
//!
//! <contextually typed row value constructor element> ::=
//!       <value expression>
//!     | <contextually typed value specification>
//! ```

use super::ValueExpression;

/// ```text
/// <contextually typed row value constructor> ::=
///       <contextually typed row value constructor element>
///     | [ ROW ]
///         <left paren>
///             <contextually typed row value constructor element list>
///         <right paren>
///
/// <contextually typed row value constructor element list> ::=
///     <contextually typed row value constructor element>
///     [ { <comma> <contextually typed row value constructor element> }... ]
///
/// <contextually typed row value constructor element> ::=
///       <value expression>
///     | <contextually typed value specification>
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct ContextuallyTypedRowValueConstructor {
    pub elements: Vec<ContextuallyTypedRowValueConstructorElement>,
}

/// ```text
/// <contextually typed row value constructor element> ::=
///       <value expression>
///     | <contextually typed value specification>
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum ContextuallyTypedRowValueConstructorElement {
    ValueExpression(ValueExpression),
}
