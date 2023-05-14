// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use strum::EnumProperty;
use thiserror::Error;

use crate::syntax::expression::QuerySpecification;

use super::{
    expression::{
        data_type::{
            DataType,
            NumericType,
            PredefinedType,
        },
        query_specification::SelectList,
        query_expression::{
            QueryExpression,
            QueryExpressionBody,
            SimpleTable,
        },
    },
    Keyword,
    Lexer,
    statement::{
        SqlDataStatement,
        SqlExecutableStatement,
        SqlSchemaDefinitionStatement,
        SqlSchemaStatement,
    },
    set_function::SetQuantifier,
    Token,
    TokenKind, schema::definition::{TableDefinition, table_definition::{TableElement, ColumnDefinition}},
};

pub struct Parser {

}

type StatementResult<'input> = Result<SqlExecutableStatement, StatementParseError<'input>>;

/// Is the token list at the end of the statement? This is expressed through
/// either EOF or the semicolon `;` token.
fn is_end_of_statement(tokens: &[Token]) -> bool {
    if tokens.is_empty() {
        true
    } else {
        tokens.len() == 1 && tokens[0].kind() == TokenKind::Semicolon
    }
}

impl Parser {
    /// Creates a new [`Parser`] object.
    pub fn new() -> Self {
        Self {
            // ...
        }
    }

    /// Parses a statement.
    pub fn parse_statement<'input>(&self, input: &'input str) -> StatementResult<'input> {
        let mut lexer = Lexer::new(input);

        let Some(first_token) = lexer.consume_token() else {
            return Err(StatementParseError::EmptyInput);
        };

        let TokenKind::Keyword(keyword) = first_token.kind() else {
            return Err(StatementParseError::StartNotAToken {
                found: first_token.as_string(input),
                token_kind: first_token.kind()
            });
        };

        let tokens: Vec<_> = lexer.collect();

        match keyword {
            Keyword::Create => self.parse_statement_create(input, &tokens),
            Keyword::Select => self.parse_statement_select(input, &tokens),

            _ => Err(StatementParseError::StartUnknownKeyword {
                found: first_token.as_string(input),
                keyword,
            })
        }
    }

    /// Parses the rest of the statement when the first token was the
    /// **`CREATE`** identifier keyword.
    fn parse_statement_create<'input>(&self, input: &'input str, tokens: &[Token]) -> StatementResult<'input> {
        if tokens.is_empty() {
            return Err(StatementParseError::EofCreateKeywordOnlyToken(input))
        }

        match tokens[0].kind() {
            TokenKind::Keyword(Keyword::Database) => return self.parse_statement_create_database(input, &tokens[1..]),
            TokenKind::Keyword(Keyword::Schema) => return self.parse_statement_create_schema(input, &tokens[1..]),
            TokenKind::Keyword(Keyword::Table) => return self.parse_statement_create_table(input, &tokens[1..]),
            TokenKind::Keyword(Keyword::View) => return self.parse_statement_create_view(input, &tokens[1..]),

            _ => Err(StatementParseError::CreateStatementUnexpectedFollowUpToken {
                found: tokens[0].as_string(input),
                token_kind: tokens[0].kind()
            })
        }
    }

    /// Parses the rest of the statement when the first two tokens were
    /// **`CREATE DATABASE`**.
    fn parse_statement_create_database<'input>(&self, input: &'input str, tokens: &[Token]) -> StatementResult<'input> {
        _ = input;
        _ = tokens;
        todo!("DATABASE creation is not yet supported")
    }

    /// Parses the rest of the statement when the first two tokens were
    /// **`CREATE SCHEMA`**.
    fn parse_statement_create_schema<'input>(&self, input: &'input str, tokens: &[Token]) -> StatementResult<'input> {
        _ = input;
        _ = tokens;
        todo!("SCHEMA creation is not yet supported")
    }

    /// Parses the rest of the statement when the first two tokens were
    /// **`CREATE TABLE`**.
    fn parse_statement_create_table<'input>(&self, input: &'input str, mut tokens: &[Token]) -> StatementResult<'input> {
        if is_end_of_statement(tokens) {
            return Err(StatementParseError::CreateTableStatementExpectedTableNameIdentifierUnexpectedEof { found: input });
        }

        let table_name = match tokens[0].kind() {
            TokenKind::Keyword(keyword) => return Err(StatementParseError::CreateTableStatementExpectedTableNameIdentifierUnexpectedKeyword{
                found: tokens[0].as_string(input),
                keyword,
            }),

            TokenKind::Identifier => {
                let name = tokens[0].as_string(input);
                tokens = &tokens[1..];
                name
            },

            _ => return Err(StatementParseError::CreateTableStatementExpectedTableNameIdentifierUnexpectedToken {
                found: tokens[0].as_string(input),
                token_kind: tokens[0].kind()
            })
        };

        if is_end_of_statement(tokens) {
            return Err(StatementParseError::CreateTableUnexpectedEofAfterTableName);
        }

        if tokens[0].kind() != TokenKind::LeftParenthesis {
            return Err(StatementParseError::CreateTableUnexpectedEofAfterTableNameAndLeftParen {
                found: tokens[0].as_string(input),

                token_kind: tokens[0].kind()
            });
        }

        let mut definition = TableDefinition {
            table_name: table_name.to_owned(),
            elements: Vec::new()
        };

        self.parse_table_elements(input, &mut tokens, &mut definition)?;

        if !is_end_of_statement(tokens) {
            return Err(StatementParseError::CreateTableUnexpectedTokenAtEnd {
                found: tokens[0].as_string(input),
                token_kind: tokens[0].kind()
            });
        }

        Ok(SqlExecutableStatement::Schema(
            SqlSchemaStatement::Definition(
                SqlSchemaDefinitionStatement::Table(
                    definition
                )
            )
        ))
    }

    /// Parses the rest of the statement when the first two tokens were
    /// **`CREATE VIEW`**.
    fn parse_statement_create_view<'input>(&self, input: &'input str, tokens: &[Token]) -> StatementResult<'input> {
        _ = input;
        _ = tokens;
        todo!("VIEW creation is not yet supported")
    }

    /// Parses the rest of the statement when the first token was the
    /// **`SELECT`** identifier keyword.
    fn parse_statement_select<'input>(&self, input: &'input str, tokens: &[Token]) -> StatementResult<'input> {
        if tokens.is_empty() {
            return Err(StatementParseError::EofSelectKeywordOnlyToken(input))
        }

        match tokens[0].kind() {
            TokenKind::Keyword(Keyword::Distinct) => return self.parse_statement_select_set_quantifier(input, &tokens[1..], SetQuantifier::Distinct),
            TokenKind::Keyword(Keyword::All) => return self.parse_statement_select_set_quantifier(input, &tokens[1..], SetQuantifier::All),
            _ => self.parse_statement_select_set_quantifier(input, tokens, SetQuantifier::default()),
        }
    }

    /// Parses the rest of the statement when the first token was the
    /// **`SELECT`** identifier keyword and the `<set quantifier>` portion
    /// was parsed.
    fn parse_statement_select_set_quantifier<'input>(&self, input: &'input str, mut tokens: &[Token], set_quantifier: SetQuantifier) -> StatementResult<'input> {
        let select_list = self.parse_select_list(input, &mut tokens)?;

        let query_specification = QuerySpecification {
            select_list,
            set_quantifier,
            table_expression: None
        };

        if !is_end_of_statement(tokens) {
            return Err(StatementParseError::SelectStatementUnexpectedToken{
                found: tokens[0].as_string(input),
                token_kind: tokens[0].kind(),
            })
        }

        // TODO this sucks. Can't we take a shortcut without code duplication?
        Ok(SqlExecutableStatement::SqlDataStatement(
            SqlDataStatement::SelectStatement(
                QueryExpression{
                    body: QueryExpressionBody::SimpleTable(
                        SimpleTable::QuerySpecification(
                            query_specification
                        )
                    )
                }
            )
        ))
    }

    /// Parses a `<select list>`
    fn parse_select_list<'input>(&self, input: &'input str, tokens: &mut &[Token]) -> Result<SelectList, StatementParseError<'input>> {
        if tokens.is_empty() {
            return Err(StatementParseError::EofSelectList(input))
        }

        if tokens[0].kind() == TokenKind::Asterisk {
            *tokens = &tokens[1..];
            return Ok(SelectList::Asterisk);
        }

        todo!("here should <value expression> be parsed")
    }

    /// Parses a single table element, but does not consume a comma `,` or
    /// closing parenthesis `)`.
    fn parse_table_element<'input, 'tokens>(&self, input: &'input str, mut tokens: &'tokens [Token])
            -> Result<(&'tokens [Token], TableElement), StatementParseError<'input>> {
        if is_end_of_statement(tokens) {
            return Err(StatementParseError::TableElementSingleUnexpectedEndOfFileAtBeginning);
        }

        // If the first token is a keyword, the user might've forgotten to
        // escape it so it becomes an identifier.
        if let TokenKind::Keyword(keyword) = tokens[0].kind() {
            return Err(StatementParseError::TableElementSingleExpectedIdentifierAsColumnNameButGotKeyword {
                found: tokens[0].as_string(input),
                keyword,
            });
        }

        if tokens[0].kind() != TokenKind::Identifier {
            return Err(StatementParseError::TableElementSingleExpectedIdentifierAsColumnName {
                found: tokens[0].as_string(input),
                token_kind: tokens[0].kind()
            });
        }

        let column_name = tokens[0].as_string(input);
        tokens = &tokens[1..];

        if is_end_of_statement(tokens) {
            return Err(StatementParseError::TableElementSingleUnexpectedEndOfFileAfterColumnName);
        }

        let data_type_keyword_token = tokens[0];
        tokens = &tokens[1..];

        let TokenKind::Keyword(data_type_keyword) = data_type_keyword_token.kind() else {
            return Err(StatementParseError::TableElementSingleExpectedKeywordAsDataType {
                found: data_type_keyword_token.as_string(input),
                token_kind: data_type_keyword_token.kind(),
            })
        };

        let data_type = match data_type_keyword {
            Keyword::Int | Keyword::Integer => DataType::Predefined(
                PredefinedType::Numeric(NumericType::Integer)
            ),
            _ => return Err(StatementParseError::TableElementSingleUnknownDataTypeKeyword {
                found: data_type_keyword_token.as_string(input),
                keyword: data_type_keyword,
            })
        };

        let column_definition = ColumnDefinition {
            column_name: column_name.to_owned(),
            data_type,
            column_constraint_definitions: Vec::new(),
        };

        Ok((tokens, TableElement::ColumnDefinition(column_definition)))
    }

    /// Parses the list of table elements, before an opening paranthesis `(` was
    /// consumed. This consumes the list of tokens up until and including the
    /// closing parenthesis `)`.
    fn parse_table_elements<'input>(&self, input: &'input str, tokens: &mut &[Token], definition: &mut TableDefinition) -> Result<(), StatementParseError<'input>> {
        if is_end_of_statement(tokens) {
            return Err(StatementParseError::TableElementsUnexpectedEndOfFileAtBeginning);
        }

        if tokens[0].kind() != TokenKind::LeftParenthesis {
            return Err(StatementParseError::TableElementsExpectedLeftParenthesis {
                found: tokens[0].as_string(input),
                token_kind: tokens[0].kind()
            });
        }

        *tokens = &tokens[1..];

        loop {
            if is_end_of_statement(tokens) {
                return Err(StatementParseError::TableElementsUnexpectedEndOfFile);
            }

            if tokens[0].kind() == TokenKind::RightParenthesis {
                if definition.elements.is_empty() {
                    return Err(StatementParseError::TableElementsUnexpectedClosingParenthesis {
                        found: tokens[0].as_string(input)
                    });
                }

                *tokens = &tokens[1..];
                return Ok(());
            }

            if tokens[0].kind() == TokenKind::Semicolon {
                return Err(StatementParseError::TableElementsUnexpectedSemicolon {
                    found: tokens[0].as_string(input)
                });
            }

            if tokens[0].kind() == TokenKind::Comma {
                if definition.elements.is_empty() {
                    return Err(StatementParseError::TableElementsUnexpectedCommaBeforeFirstColumn {
                        found: tokens[0].as_string(input),
                    });
                }

                if is_end_of_statement(&tokens[1..]) {
                    return Err(StatementParseError::TableElementsUnexpectedEndOfFileAfterComma {
                        found: tokens[0].as_string(input),
                    });
                }

                *tokens = &tokens[1..];
            }

            let element_pair = self.parse_table_element(input, tokens)?;
            *tokens = element_pair.0;

            definition.elements.push(element_pair.1);
        }
    }
}

/// Describes an error in parsing a statement.
#[derive(Copy, Clone, Debug, Error, PartialEq, EnumProperty)]
pub enum StatementParseError<'input> {
    #[error("unexpected keyword: {token_kind:?}: `{found}`.\n`CREATE` keyword not followed by either TABLE, VIEW, SCHEMA or DATABASE")]
    CreateStatementUnexpectedFollowUpToken {
        found: &'input str,
        token_kind: TokenKind,
    },

    #[error("unexpected end-of-file: `CREATE TABLE` not followed by the name of the table to create")]
    CreateTableStatementExpectedTableNameIdentifierUnexpectedEof {
        found: &'input str,
    },

    #[error("unexpected keyword: `{keyword}` (`{found}`): expected an identifier as the name of the table to create.")]
    #[strum(props(Hint="Did you forget to escape the identifier?"))]
    CreateTableStatementExpectedTableNameIdentifierUnexpectedKeyword {
        found: &'input str,
        keyword: Keyword,
    },

    #[error("unexpected token: `{token_kind}` (`{found}`): expected an identifier as the name of the table to create.")]
    CreateTableStatementExpectedTableNameIdentifierUnexpectedToken {
        found: &'input str,
        token_kind: TokenKind,
    },

    #[error("unexpected end-of-file: expected '(' after table name of `CREATE TABLE`")]
    CreateTableUnexpectedEofAfterTableName,

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

    #[error("empty input provided for statement")]
    EmptyInput,

    #[error("unexpected end-of-file: `CREATE` keyword not followed by either TABLE, VIEW, SCHEMA or DATABASE")]
    EofCreateKeywordOnlyToken(&'input str),

    #[error("unexpected end-of-file: `SELECT` keyword not followed by either an <set quantifier> or <select list>")]
    EofSelectKeywordOnlyToken(&'input str),

    #[error("unexpected end-of-file: expected a <select list>, but end of statement reached. Expected either the wildcard '*' expression or a <value expression>")]
    EofSelectList(&'input str),

    #[error("unexpected token {token_kind:?}: `{found}`")]
    SelectStatementUnexpectedToken {
        found: &'input str,
        token_kind: TokenKind,
    },

    #[error("statement doesn't start with a keyword, but a {token_kind:?}: `{found}`")]
    StartNotAToken {
        found: &'input str,
        token_kind: TokenKind,
    },

    #[error("unexpected keyword {keyword:?} as start of statement: `{found}`")]
    StartUnknownKeyword {
        found: &'input str,
        keyword: Keyword,
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

    #[error("unexpected keyword: {keyword} (`{found}`), expected identifier as column name")]
    #[strum(props(Hint="Did you forget to escape the column name?"))]
    #[strum(props(Help="`{keyword}` is reserved as a keyword"))]
    TableElementSingleExpectedIdentifierAsColumnNameButGotKeyword {
        found: &'input str,
        keyword: Keyword,
    },

    #[error("unexpected end-of-file after the column name")]
    #[strum(props(Help="Follow the column name with the column type, e.g. `INT`, `NVARCHAR`, etc."))]
    TableElementSingleUnexpectedEndOfFileAfterColumnName,

    #[error("unexpected end-of-file at the beginning of <table element>")]
    TableElementSingleUnexpectedEndOfFileAtBeginning,

    #[error("unknown keyword `{found}`, expected data type for column definition")]
    TableElementSingleUnknownDataTypeKeyword {
        found: &'input str,
        keyword: Keyword,
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
    TableElementsUnexpectedEndOfFile,

    #[error("unexpected end-of-file after the comma that is separating column definitions: `{found}`")]
    TableElementsUnexpectedEndOfFileAfterComma {
        found: &'input str,
    },

    #[error("unexpected end-of-file before table column definition list")]
    #[strum(props(Hint="Did you forget to add the column definitions?"))]
    #[strum(props(Help="Supply the column definitions by opening with a parenthesis `(`, followed by one or more columns (e.g. `id INT`) and closing with a parenthesis `)`. For example: `CREATE TABLE my_favorite_numbers (value INT);"))]
    TableElementsUnexpectedEndOfFileAtBeginning,

    #[error("unexpected semicolon `{found}` in the table column definitions")]
    #[strum(props(Hint="Did you forget to close the column list by inserting a closing parenthesis `)`?"))]
    TableElementsUnexpectedSemicolon {
        found: &'input str,
    },
}

#[cfg(test)]
mod tests {
    use std::ops::{
        Range,
        RangeFrom,
    };

    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("CREATE TABLE table (id INT);", Keyword::Table, 13..18)]
    #[case("CREATE TABLE character (value INT);", Keyword::Character, 13..22)]
    fn parser_create_table_statement_keyword_as_table_name(#[case] input: &str, #[case] keyword: Keyword, #[case] range: Range<usize>) {
        let expected = StatementParseError::CreateTableStatementExpectedTableNameIdentifierUnexpectedKeyword {
            found: &input[range],
            keyword,
        };

        assert_eq!(Parser::new().parse_statement(input), Err(expected));
    }

    #[rstest]
    #[case("CREATE TABLE blog_posts (my_id INT)")]
    fn parser_simple_create_table_statement(#[case] input: &str) {
        let parser = Parser::new();

        parser.parse_statement(input).unwrap();
    }

    #[rstest]
    #[case("SELECT *")]
    #[case("SELECT *;")]
    #[case("select *")]
    #[case("select *;")]
    #[case("   select  *  ; ")]
    #[case("  SElect     * ;   ")]
    fn parser_simple_select_statement(#[case] input: &str) {
        let parser = Parser::new();

        let expected = SqlExecutableStatement::SqlDataStatement(
            SqlDataStatement::SelectStatement(
                QueryExpression{
                    body: QueryExpressionBody::SimpleTable(
                        SimpleTable::QuerySpecification(
                            QuerySpecification {
                                set_quantifier: SetQuantifier::All,
                                select_list: SelectList::Asterisk,
                                table_expression: None,
                            }
                        )
                    )
                }
            )
        );

        assert_eq!(parser.parse_statement(input), Ok(expected));
    }

    #[rstest]
    #[case("SELECT *,", TokenKind::Comma, 8..)]
    #[case("SELECT * *", TokenKind::Asterisk, 9..)]
    #[case("SELECT * select", TokenKind::Keyword(Keyword::Select), 9..)]
    fn parser_invalid_tokens_after_simple_select(#[case] input: &str, #[case] token_kind: TokenKind, #[case] range: RangeFrom<usize>) {
        let parser = Parser::new();

        assert_eq!(
            parser.parse_statement(input),
            Err(
                StatementParseError::SelectStatementUnexpectedToken {
                    found: &input[range],
                    token_kind,
                }
            )
        );
    }

}
