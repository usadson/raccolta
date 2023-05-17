// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

//! ISO/IEC 9075-2:1999 - Framework - 7.3 <table value constructor>
//!
//! # Definitions
//! ```text
//! <table value constructor> ::= VALUES <row value expression list>
//!
//! <row value expression list> ::= <row value expression> [ { <comma> <row value expression> }... ]
//!
//! <contextually typed table value constructor> ::= VALUES <contextually typed row value expression list>
//!
//! <contextually typed row value expression list> ::=
//!     <contextually typed row value expression>
//!     [ { <comma> <contextually typed row value expression> }... ]
//! ```

use crate::expression::row_value_expression::ContextuallyTypedRowValueExpression;

/// ```text
/// <contextually typed table value constructor> ::=
///     VALUES <contextually typed row value expression list>
///
/// <contextually typed row value expression list> ::=
///     <contextually typed row value expression>
///     [ { <comma> <contextually typed row value expression> }... ]
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct ContextuallyTypedTableValueConstructor {
    pub values: Vec<ContextuallyTypedRowValueExpression>,
}
