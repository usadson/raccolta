// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

//! ```text
//! <table value constructor> ::=
//!     VALUES <row value expression list>
//!
//! <row value expression list> ::=
//!     <row value expression> [ { <comma> <row value expression> }... ]
//!
//! <contextually typed table value constructor> ::=
//!     VALUES <contextually typed row value expression list>
//!
//! <contextually typed row value expression list> ::=
//!     <contextually typed row value expression>
//!     [ { <comma> <contextually typed row value expression> }... ]
//!
//!
//! <contextually typed row value expression> ::=
//!       <row value special case>
//!     | <contextually typed row value constructor>
//!
//! <row value special case> ::=
//!       <value specification>
//!     | <value expression>
//!
//!
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
//!
//! <contextually typed value specification> ::=
//!       <implicitly typed value specification>
//!     | <default specification>
//!
//! <implicitly typed value specification> ::=
//!       <null specification>
//!     | <empty specification>
//!
//! <null specification> ::= NULL
//! <default specification> ::= DEFAULT
//!
//! <empty specification> ::= ARRAY <left bracket or trigraph> <right bracket or trigraph>
//! ```

use crate::common::TableName;

/// ```text
//// <insert columns and source> ::=
////      <from subquery>
////    | <from constructor>
////    | <from default>
///
//// <from subquery> ::=
////    [ <left paren> <insert column list> <right paren> ]
////    [ override clause> ]
////    <query expression>
///
//// <from constructor> ::=
////    [ <left paren> <insert column list> <right paren> ]
////    [ <override clause> ]
////    <contextually typed table value constructor>
///
//// <override clause> ::=
////      OVERRIDING USER VALUE
////    | OVERRIDING SYSTEM VALUE
///
//// <from default> ::=
////    DEFAULT VALUES
///
//// <insert column list> ::= <column name list>
///
/// <column name list> ::=
///     <column name> [ { <comma> <column name> }... ]
///
/// <column name> ::=
////    <identifier>
///  ```
#[derive(Clone, Debug, PartialEq)]
pub enum InsertColumnsAndSource {
    FromConstructor {
        insert_column_list: Option<Vec<String>>,
    }
}

/// ```text
/// <insert statement> ::=
///     INSERT INTO <insertion target>
///     <insert columns and source>
///
/// <insertion target> ::=
////    <table name>
/// ```
///
/// # Example
/// In the example below:
/// ```sql
/// INSERT INTO table1 (num, val)
/// VALUES (1, 'Hello'),
///        (2, 'World')
/// ```
///
/// This is evaluated to the following:
/// ```text
/// <insert columns and source>
///     <from constructor>
///         <left paren> <insert column list> <right paren>
///             <column name list>
///                 <column name>
///                     <identifier>
///                         "num"
///                 <comma>
///                 <column name>
///                     <identifier>
///                         "val"
///         <contextually typed table value constructor>
///             VALUES <contextually typed row value expression list>
///                 <contextually typed row value expression list>
///                     <contextually typed row value expression>
///                         <contextually typed row value constructor>
///                             <left paren>
///                             <contextually typed row value constructor element list>
///                                 <contextually typed row value constructor element>
///                                     <value expression>
///                                         ...
///                                             1
///                                 <comma>
///                                 <contextually typed row value constructor element>
///                                     <value expression>
///                                         ...
///                                             "Hello"
///                             <right paren>
///                     <comma>
///                     <contextually typed row value expression>
///                         <contextually typed row value constructor>
///                             <left paren>
///                             <contextually typed row value constructor element list>
///                                 <contextually typed row value constructor element>
///                                     <value expression>
///                                         ...
///                                             2
///                                 <comma>
///                                 <contextually typed row value constructor element>
///                                     <value expression>
///                                         ...
///                                             "World"
///                             <right paren>
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct InsertStatement {
    /// The table name wherein data must be inserted.
    ///
    /// `<insertion target>` i.e. `<table name>`
    pub table_name: TableName,

    /// `<insert columns and source>`
    ///
    /// ## Explanation
    /// Commonly, this consists of two parts:
    /// 1. `<insert column list>`
    /// 2. `<table value constructor>`
    pub insert_columns_and_source: InsertColumnsAndSource,
}
