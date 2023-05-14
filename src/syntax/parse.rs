// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use strum::EnumProperty;
use thiserror::Error;

use crate::syntax::expression::QuerySpecification;

use super::{
    expression::{
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
    },
    set_function::SetQuantifier,
    Token,
    TokenKind,
};

pub struct Parser {

}

type StatementResult<'input> = Result<SqlExecutableStatement, StatementParseError<'input>>;

/// Is the token list at the end of the statement? This is expressed through
/// either EOF or the semicolon ';' token.
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
    fn parse_statement_create_table<'input>(&self, input: &'input str, tokens: &[Token]) -> StatementResult<'input> {
        if is_end_of_statement(tokens) {
            return Err(StatementParseError::CreateTableStatementExpectedTableNameIdentifierUnexpectedEof { found: input });
        }

        match tokens[0].kind() {
            TokenKind::Keyword(keyword) => Err(StatementParseError::CreateTableStatementExpectedTableNameIdentifierUnexpectedKeyword{
                found: tokens[0].as_string(input),
                keyword,
            }),

            _ => Err(StatementParseError::CreateTableStatementExpectedTableNameIdentifierUnexpectedToken {
                found: tokens[0].as_string(input),
                token_kind: tokens[0].kind()
            })
        }
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
}

/// Describes an error in parsing a statement.
#[derive(Clone, Debug, Error, PartialEq, EnumProperty)]
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
    #[case("CREATE TABLE character (value INT);", Keyword::Character, 13..21)]
    fn parser_create_table_statement_keyword_as_table_name(#[case] input: &str, #[case] keyword: Keyword, #[case] range: Range<usize>) {
        let expected = StatementParseError::CreateTableStatementExpectedTableNameIdentifierUnexpectedKeyword {
            found: &input[range],
            keyword,
        };

        assert_eq!(Parser::new().parse_statement(input), Err(expected));
    }

    #[rstest]
    #[case("CREATE TABLE blog_posts (id INT)")]
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
