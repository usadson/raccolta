// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use strum::{
    AsRefStr,
    EnumProperty,
};

use thiserror::Error;

use crate::{
    ReservedWord,
    TokenKind,
};

/// Describes an error in parsing a statement.
///
/// # To Do
/// TODO: it would be very cool to match these parenthesis, something like:
/// ```text
/// INSERT INTO table1
/// VALUES (1, 2
///        ^    ^ error occurred here
///        |
///        `- to match this value here
/// ```
#[derive(Copy, Clone, Debug, Error, PartialEq, EnumProperty, AsRefStr, enum_fields::EnumFields)]
pub enum StatementParseError<'input> {
    #[error("unexpected end-of-file: expected column reference")]
    ColumnReferenceUnexpectedEndOfFile {
        found: &'input str,
    },

    #[error("unexpected token: {token_kind} (`{found}`), expected identifier as column reference")]
    ColumnReferenceUnexpectedToken {
        found: &'input str,
        token_kind: TokenKind,
    },

    #[error("unexpected end-of-file: expected `(` to start contextually typed row value constructor")]
    #[strum(props(Hint="Did you forget to add a row value constructor, or mistyped the last comma `,`?"))]
    ContextuallyTypedRowValueConstructorUnexpectedEndOfFile {
        found: &'input str,
    },

    #[error("unexpected end-of-file: expected `)` to end the row value constructor")]
    #[strum(props(Hint="Did you forget to add a row value constructor, or mistyped the last comma `,`?"))]
    ContextuallyTypedRowValueConstructorUnexpectedEndOfFileExpectedCommaOrRightParen {
        found: &'input str,
    },

    #[error("unexpected token: {token_kind} (`{found}`), expected comma `,` or closing parenthesis `(`")]
    #[strum(props(Hint="Did you forget to add a comma `,` to add another column, or `)` to close this row?"))]
    ContextuallyTypedRowValueConstructorUnexpectedTokenExpectedCommaOrRightParen {
        found: &'input str,
        token_kind: TokenKind,
    },

    #[error("unexpected token: {token_kind} (`{found}`), expected opening parenthesis `(`")]
    #[strum(props(Hint="Did you forget to add a row value constructor, or mistyped the last comma `,`?"))]
    ContextuallyTypedRowValueConstructorUnexpectedTokenExpectedLeftParen {
        found: &'input str,
        token_kind: TokenKind,
    },

    #[error("unexpected end-of-file: expected identifier as the correlation name (alias)")]
    CorrelationNameUnexpectedEndOfFile {
        found: &'input str,
    },

    #[error("unexpected keyword: `{reserved_word}` (`{found}`): expected an identifier as the name of the correlation name (alias)")]
    #[strum(props(Hint="Did you forget to escape the identifier?"))]
    CorrelationNameUnexpectedReservedWord {
        found: &'input str,
        reserved_word: ReservedWord,
    },

    #[error("unexpected token: `{token_kind}` (`{found}`): expected the name of the correlation name (alias)")]
    CorrelationNameUnexpectedToken {
        found: &'input str,
        token_kind: TokenKind
    },

    #[error("unexpected keyword: {token_kind:?}: `{found}`")]
    #[strum(props(Help="`CREATE` keyword not followed by either TABLE, VIEW, SCHEMA or DATABASE"))]
    CreateStatementUnexpectedFollowUpToken {
        found: &'input str,
        token_kind: TokenKind,
    },

    #[error("unexpected end-of-file: `CREATE TABLE` not followed by the name of the table to create")]
    CreateTableStatementExpectedTableNameIdentifierUnexpectedEof {
        found: &'input str,
    },

    #[error("unexpected keyword: `{reserved_word}` (`{found}`): expected an identifier as the name of the table to create.")]
    #[strum(props(Hint="Did you forget to escape the identifier?"))]
    CreateTableStatementExpectedTableNameIdentifierUnexpectedKeyword {
        found: &'input str,
        reserved_word: ReservedWord,
    },

    #[error("unexpected token: `{token_kind}` (`{found}`): expected an identifier as the name of the table to create.")]
    CreateTableStatementExpectedTableNameIdentifierUnexpectedToken {
        found: &'input str,
        token_kind: TokenKind,
    },

    #[error("unexpected end-of-file: expected '(' after table name of `CREATE TABLE`")]
    CreateTableUnexpectedEofAfterTableName  {
        found: &'input str,
    },

    #[error("unexpected end-of-file: expected <table element> after '(' of `CREATE TABLE`, got: {found} (`{token_kind}`)")]
    CreateTableUnexpectedEofAfterTableNameAndLeftParen {
        found: &'input str,
        token_kind: TokenKind,
    },

    #[error("unexpected token in `CREATE TABLE` after <table element list>: {token_kind} (`{found}`)")]
    CreateTableUnexpectedTokenAtEnd {
        found: &'input str,
        token_kind: TokenKind,
    },

    #[error("unexpected end-of-file: expected `(` after `VARCHAR`")]
    #[strum(props(Help="Complete the VARCHAR data type: `VARCHAR( <maximum length> )`"))]
    DataTypeVarcharUnexpectedEndOfFileExpectedLeftParen {
        found: &'input str,
    },

    #[error("unexpected token-end-of-file: expected a number indicating the maximum length of `VARCHAR`")]
    #[strum(props(Help="Complete the VARCHAR data type: `VARCHAR( <maximum length> )`"))]
    DataTypeVarcharUnexpectedEndOfFileExpectedLength {
        found: &'input str,
    },

    #[error("unexpected end-of-file: expected closing parenthesis `)`")]
    #[strum(props(Help="Complete the VARCHAR data type: `VARCHAR({length})`"))]
    DataTypeVarcharUnexpectedEndOfFileExpectedRightParen {
        found: &'input str,
        length: usize,
    },

    #[error("unexpected token: {token_kind} (`{found}`), expected `(` after `VARCHAR`")]
    #[strum(props(Help="Complete the VARCHAR data type: `VARCHAR( <maximum length> )`"))]
    DataTypeVarcharUnexpectedTokenExpectedLeftParen {
        found: &'input str,
        token_kind: TokenKind,
    },

    #[error("unexpected token: {token_kind} (`{found}`), expected a number indicating the maximum length")]
    #[strum(props(Help="Complete the VARCHAR data type: `VARCHAR( <maximum length> )`"))]
    DataTypeVarcharUnexpectedTokenExpectedLength {
        found: &'input str,
        token_kind: TokenKind,
    },

    #[error("unexpected token: {token_kind} (`{found}`), expected `)` after `VARCHAR({length}`")]
    #[strum(props(Help="Complete the VARCHAR data type: `VARCHAR({length})`"))]
    DataTypeVarcharUnexpectedTokenExpectedRightParen {
        found: &'input str,
        token_kind: TokenKind,
        length: usize,
    },

    #[error("empty input provided for statement")]
    EmptyInput,

    #[error("unexpected end-of-file: `CREATE` keyword not followed by either TABLE, VIEW, SCHEMA or DATABASE")]
    EofCreateKeywordOnlyToken(&'input str),

    #[error("unexpected end-of-file: `SELECT` keyword not followed by either an <set quantifier> or <select list>")]
    EofSelectKeywordOnlyToken(&'input str),

    #[error("unexpected end-of-file: expected a <select list>, but end of statement reached. Expected either the wildcard '*' expression or a <value expression>")]
    EofSelectList(&'input str),

    #[error("unexpected end-of-file, expected FROM clause")]
    FromClauseUnexpectedEof  {
        found: &'input str,
    },

    #[error("unexpected token {token_kind}: `{found}`, expected FROM clause")]
    FromClauseUnexpectedToken {
        found: &'input str,
        token_kind: TokenKind,
    },

    /// TODO: when column source information is parsed, this should be changed
    ///       to also hint at the insertion of `( column name, ... )`
    #[error("unexpected end-of-file, expected `VALUES` keyword")]
    #[strum(props(Help="Insert the `VALUES` keyword, followed by one or more column data lists"))]
    InsertColumnsAndSourceUnexpectedEndOfFileAtBeginning {
        found: &'input str,
    },

    #[error("unexpected keyword {reserved_word} (`{found}`), expected `VALUES` keyword")]
    #[strum(props(Help="Replace this with the `VALUES` keyword"))]
    InsertColumnsAndSourceUnexpectedKeyword {
        found: &'input str,
        reserved_word: ReservedWord,
    },

    #[error("unexpected token {token_kind} (`{found}`), expected `VALUES` keyword")]
    #[strum(props(Hint="Did you forget to begin the rows with the `VALUES` keyword?"))]
    #[strum(props(Help="Insert the `VALUES` keyword, followed by one or more column data lists"))]
    InsertColumnsAndSourceUnexpectedToken {
        found: &'input str,
        token_kind: TokenKind,
    },

    #[error("unexpected end-of-file, expected `INTO` keyword")]
    #[strum(props(Hint="`INSERT` on its own isn't a statement, but it is the start of the `INSERT INTO` statement, which allows you to insert one or more rows into a table"))]
    #[strum(props(Help="Append the `INTO` keyword: `INSERT INTO`"))]
    InsertStatementEndOfFile {
        found: &'input str,
    },

    #[error("unexpected keyword {reserved_word} (`{found}`), expected `INTO` keyword")]
    #[strum(props(Hint="`INSERT` must be followed by the `INTO` keyword."))]
    #[strum(props(Help="Replace it with the `INTO` keyword: `INSERT INTO`"))]
    InsertStatementUnexpectedKeyword {
        found: &'input str,
        reserved_word: ReservedWord,
    },

    #[error("unexpected token {token_kind} (`{found}`), expected `INTO` keyword")]
    #[strum(props(Hint="`INSERT` must be followed by the `INTO` keyword."))]
    #[strum(props(Help="Insert the `INTO` keyword: `INSERT INTO`"))]
    InsertStatementUnexpectedToken {
        found: &'input str,
        token_kind: TokenKind,
    },

    #[error("unexpected end-of-file, expected the name of the table to insert into")]
    #[strum(props(Help="Append the a table name wherein you want to insert data to."))]
    InsertIntoStatementEndOfFile {
        found: &'input str,
    },

    #[error("unexpected token {token_kind} (`{found}`), expected the name of the table to insert into")]
    #[strum(props(Help="Append the a table name wherein you want to insert data to: `INSERT INTO table_name_here`"))]
    InsertIntoStatementUnexpectedToken {
        found: &'input str,
        token_kind: TokenKind,
    },

    #[error("unexpected trailing token in `INSERT TABLE`: {token_kind} (`{found}`)")]
    #[strum(props(Hint="This isn't a known clause of the `INSERT INTO` statement."))]
    InsertIntoStatementUnexpectedTrailingToken {
        found: &'input str,
        token_kind: TokenKind,
    },

    #[error("unexpected end-of-file after `ORDER`, expected `BY`")]
    OrderByClauseUnexpectedEndOfFileExpectedBy {
        found: &'input str,
    },

    #[error("unexpected end-of-file, expected `ORDER`")]
    OrderByClauseUnexpectedEndOfFileExpectedOrder {
        found: &'input str,
    },

    #[error("unexpected token: {token_kind} (`{found}`), expected `BY` after `ORDER`")]
    OrderByClauseUnexpectedTokenExpectedBy {
        found: &'input str,
        token_kind: TokenKind
    },

    #[error("unexpected token: {token_kind} (`{found}`), expected `ORDER`")]
    OrderByClauseUnexpectedTokenExpectedOrder {
        found: &'input str,
        token_kind: TokenKind
    },

    #[error("unexpected token {token_kind}: `{found}`")]
    SelectStatementUnexpectedToken {
        found: &'input str,
        token_kind: TokenKind,
    },

    #[error("unexpected token: {token_kind} (`{found}`), expected `*` after `COUNT(`")]
    #[strum(props(Help="Complete the COUNT set function specification: `COUNT(*)`"))]
    SetFunctionSpecificationCountUnexpectedTokenExpectedAsterisk {
        found: &'input str,
        token_kind: TokenKind,
    },

    #[error("unexpected token: {token_kind} (`{found}`), expected `(` after `COUNT`")]
    #[strum(props(Help="Complete the COUNT set function specification: `COUNT(*)`"))]
    SetFunctionSpecificationCountUnexpectedTokenExpectedLeftParen {
        found: &'input str,
        token_kind: TokenKind,
    },

    #[error("unexpected token: {token_kind} (`{found}`), expected `)` after `COUNT(*`")]
    #[strum(props(Help="Complete the COUNT set function specification: `COUNT(*)`"))]
    SetFunctionSpecificationCountUnexpectedTokenExpectedRightParen {
        found: &'input str,
        token_kind: TokenKind,
    },

    #[error("unexpected end-of-file, expected `*` after `COUNT(`")]
    #[strum(props(Help="Complete the COUNT set function specification: `COUNT(*)`"))]
    SetFunctionSpecificationCountUnexpectedEofExpectedAsterisk {
        found: &'input str,
    },

    #[error("unexpected end-of-file, expected `(` after `COUNT`")]
    #[strum(props(Help="Complete the COUNT set function specification: `COUNT(*)`"))]
    SetFunctionSpecificationCountUnexpectedEofExpectedLeftParen {
        found: &'input str,
    },

    #[error("unexpected end-of-file, expected `)` after `COUNT(*`")]
    #[strum(props(Help="Complete the COUNT set function specification: `COUNT(*)`"))]
    SetFunctionSpecificationCountUnexpectedEofExpectedRightParen {
        found: &'input str,
    },

    #[error("statement doesn't start with a keyword, but a {token_kind:?}: `{found}`")]
    StartNotAToken {
        found: &'input str,
        token_kind: TokenKind,
    },

    #[error("unexpected keyword {reserved_word:?} as start of statement: `{found}`")]
    StartUnknownKeyword {
        found: &'input str,
        reserved_word: ReservedWord,
    },

    #[error("unexpected token: {token_kind} (`{found}`), expected keyword as column data type")]
    #[strum(props(Help="Follow the column name with the column type instead of this token, e.g. `INT`, `NVARCHAR`, etc."))]
    TableElementSingleExpectedKeywordAsDataType {
        found: &'input str,
        token_kind: TokenKind,
    },

    #[error("unexpected token: {token_kind} (`{found}`), expected identifier as column name")]
    #[strum(props(Help="A column definition must start with the name of the column"))]
    TableElementSingleExpectedIdentifierAsColumnName {
        found: &'input str,
        token_kind: TokenKind,
    },

    #[error("unexpected keyword: {reserved_word} (`{found}`), expected identifier as column name")]
    #[strum(props(Hint="Did you forget to escape the column name?"))]
    #[strum(props(Help="`{reserved_word}` is reserved as a keyword"))]
    TableElementSingleExpectedIdentifierAsColumnNameButGotKeyword {
        found: &'input str,
        reserved_word: ReservedWord,
    },

    #[error("unexpected end-of-file after the column name")]
    #[strum(props(Help="Follow the column name with the column type, e.g. `INT`, `NVARCHAR`, etc."))]
    TableElementSingleUnexpectedEndOfFileAfterColumnName {
        found: &'input str,
    },

    #[error("unexpected end-of-file at the beginning of <table element>")]
    TableElementSingleUnexpectedEndOfFileAtBeginning {
        found: &'input str,
    },

    #[error("unknown keyword {reserved_word} (`{found}`), expected data type for column definition")]
    TableElementSingleUnknownDataTypeKeyword {
        found: &'input str,
        reserved_word: ReservedWord,
    },

    #[error("unexpected token {token_kind} (`{found}`), expected opening parenthesis `(` for opening the column list")]
    #[strum(props(Help="The column list was expected and is started by a parenthesis `(`, followed by one or more columns"))]
    TableElementsExpectedLeftParenthesis {
        found: &'input str,
        token_kind: TokenKind,
    },

    #[error("unexpected closing parenthesis `{found}` in the table column definition")]
    #[strum(props(Hint="Tables must contain at least one column."))]
    TableElementsUnexpectedClosingParenthesis {
        found: &'input str,
    },

    #[error("unexpected comma `{found}` before the first table column definition")]
    #[strum(props(Hint="While column definitions are in fact separated with commas, they mustn't appear before the first or after the last column definition."))]
    #[strum(props(Help="Remove this comma."))]
    TableElementsUnexpectedCommaBeforeFirstColumn {
        found: &'input str,
    },

    #[error("unexpected end-of-file in the table column definitions")]
    #[strum(props(Hint="Did you forget to close the column list by inserting a closing parenthesis `)`?"))]
    TableElementsUnexpectedEndOfFile {
        found: &'input str,
    },

    #[error("unexpected end-of-file after the comma that is separating column definitions: `{found}`")]
    TableElementsUnexpectedEndOfFileAfterComma {
        found: &'input str,
    },

    #[error("unexpected end-of-file before table column definition list")]
    #[strum(props(Hint="Did you forget to add the column definitions?"))]
    #[strum(props(Help="Supply the column definitions by opening with a parenthesis `(`, followed by one or more columns (e.g. `id INT`) and closing with a parenthesis `)`. For example: `CREATE TABLE my_favorite_numbers (value INT);"))]
    TableElementsUnexpectedEndOfFileAtBeginning {
        found: &'input str,
    },

    #[error("unexpected semicolon `{found}` in the table column definitions")]
    #[strum(props(Hint="Did you forget to close the column list by inserting a closing parenthesis `)`?"))]
    TableElementsUnexpectedSemicolon {
        found: &'input str,
    },

    #[error("unexpected end-of-file, expected a table reference")]
    #[strum(props(Hint="Did you forget to add a table name?"))]
    TableReferenceUnexpectedEndOfFile {
        found: &'input str,
    },

    #[error("unexpected keyword: {reserved_word} (`{found}`), expected a table reference")]
    #[strum(props(Hint="Did you forget to escape the table or schema name?"))]
    #[strum(props(Help="`{reserved_word}` is reserved as a keyword"))]
    TableReferenceUnexpectedKeyword {
        found: &'input str,
        reserved_word: ReservedWord,
    },

    #[error("unexpected token: {token_kind} (`{found}`), expected a table reference")]
    TableReferenceUnexpectedToken {
        found: &'input str,
        token_kind: TokenKind,
    },

    #[error("unsupported feature: {feature_name}, found at: {token_kind} (`{found}`)")]
    #[strum(props(Help="You can create an issue at: https://github.com/usadson/raccolta/issues/new?template=bug_report.md"))]
    UnsupportedFeature {
        feature_name: &'static str,
        feature_description: &'static str,
        found: &'input str,
        token_kind: TokenKind,
    },

    #[error("unexpected end-of-file: expected a column name, value or expression")]
    ValueExpressionUnexpectedEndOfFile {
        found: &'input str,
    },

    #[error("unexpected token: {token_kind} (`{found}`), expected a column name, value or expression")]
    ValueExpressionUnexpectedToken {
        found: &'input str,
        token_kind: TokenKind,
    },
}
