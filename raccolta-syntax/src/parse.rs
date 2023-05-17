// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod extensions;

use strum::{EnumProperty, AsRefStr};
use thiserror::Error;

use crate::{
    clause::FromClause,
    common::TableName,
    expression::{
        data_type::{
            DataType,
            NumericType,
            PredefinedType,
        },
        NumericValueExpression,
        query_specification::{
            QuerySpecification,
            DerivedColumn,
            SelectList,
            SelectSublist,
        },
        query_expression::{
            QueryExpression,
            QueryExpressionBody,
            SimpleTable,
        },
        row_value_constructor::{ContextuallyTypedRowValueConstructor, ContextuallyTypedRowValueConstructorElement},
        row_value_expression::ContextuallyTypedRowValueExpression,
        TableExpression,
        table_reference::{
            TablePrimary,
            TablePrimaryKind,
            TableReference,
        },
        table_value_constructor::ContextuallyTypedTableValueConstructor,
        ValueExpression,
    },
    Keyword,
    Lexer,
    schema::definition::table_definition::{
        ColumnDefinition,
        TableDefinition,
        TableElement,
    },
    set_function::SetQuantifier,
    statement::{
        insert_statement::{
            InsertColumnsAndSource,
            InsertStatement,
        },
        SqlDataChangeStatement,
        SqlDataStatement,
        SqlExecutableStatement,
        SqlSchemaDefinitionStatement,
        SqlSchemaStatement,
    },
    Token,
    TokenKind,
};

use extensions::ParseArrayExtensions;

use self::extensions::ParseStringExtensions;

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
    /// ```
    fn parse_contextually_typed_row_value_constructor<'input>(&self, input: &'input str, tokens: &mut &[Token]) -> Result<ContextuallyTypedRowValueConstructor, StatementParseError<'input>> {
        if is_end_of_statement(tokens) {
            return Err(StatementParseError::ContextuallyTypedRowValueConstructorUnexpectedEndOfFile {
                found: input.slice_empty_end(),
            });
        }

        if tokens[0].kind() != TokenKind::LeftParenthesis {
            return Err(StatementParseError::ContextuallyTypedRowValueConstructorUnexpectedTokenExpectedLeftParen {
                found: tokens[0].as_string(input),
                token_kind: tokens[0].kind()
            });
        }

        *tokens = &tokens[1..];

        let mut constructor = ContextuallyTypedRowValueConstructor {
            elements: Vec::new(),
        };

        loop {
            let element = self.parse_contextually_typed_row_value_constructor_element(input, tokens)?;
            constructor.elements.push(element);

            if is_end_of_statement(tokens) {
                return Err(StatementParseError::ContextuallyTypedRowValueConstructorUnexpectedEndOfFileExpectedCommaOrRightParen {
                    found: input.slice_empty_end(),
                });
            }

            let separator_token = tokens[0];
            *tokens = &tokens[1..];

            match separator_token.kind() {
                TokenKind::Comma => continue,
                TokenKind::RightParenthesis => break,
                _ => return Err(StatementParseError::ContextuallyTypedRowValueConstructorUnexpectedTokenExpectedCommaOrRightParen {
                    found: separator_token.as_string(input),
                    token_kind: separator_token.kind()
                })
            }
        }

        Ok(constructor)
    }

    /// ```text
    /// <contextually typed row value constructor element> ::=
    ///       <value expression>
    ///     | <contextually typed value specification>
    /// ```
    fn parse_contextually_typed_row_value_constructor_element<'input>(&self, input: &'input str, tokens: &mut &[Token]) -> Result<ContextuallyTypedRowValueConstructorElement, StatementParseError<'input>> {
        Ok(ContextuallyTypedRowValueConstructorElement::ValueExpression(
            self.parse_value_expression(input, tokens)?
        ))
    }

    /// ```text
    /// <contextually typed row value expression> ::=
    ///       <row value special case>
    ///     | <contextually typed row value constructor>
    ///
    /// <row value special case> ::=
    ///       <value specification>
    ///     | <value expression>
    /// ```
    fn parse_contextually_typed_row_value_expression<'input>(&self, input: &'input str, tokens: &mut &[Token]) -> Result<ContextuallyTypedRowValueExpression, StatementParseError<'input>> {
        Ok(ContextuallyTypedRowValueExpression::ContextuallyTypedRowValueConstructor(
            self.parse_contextually_typed_row_value_constructor(input, tokens)?
        ))
    }

    /// After the `VALUES` keyword was consumed, this function is invoked by
    /// [`Parser::parse_insert_columns_and_source()`].
    ///
    /// ```text
    /// <contextually typed table value constructor> ::=
    ///     VALUES <contextually typed row value expression list>
    ///
    /// <contextually typed row value expression list> ::=
    ///     <contextually typed row value expression>
    ///     [ { <comma> <contextually typed row value expression> }... ]
    /// ```
    fn parse_contextually_typed_table_value_constructor<'input>(&self, input: &'input str, tokens: &mut &[Token]) -> Result<ContextuallyTypedTableValueConstructor, StatementParseError<'input>> {
        let mut constructor = ContextuallyTypedTableValueConstructor {
            values: Vec::new(),
        };

        loop {
            constructor.values.push(self.parse_contextually_typed_row_value_expression(input, tokens)?);

            if is_end_of_statement(tokens) {
                break;
            }

            if tokens[0].kind() != TokenKind::Comma {
                break;
            }

            *tokens = &tokens[1..];
        }

        Ok(constructor)
    }

    /// Parse an optional `<correlation name>`. A correlation name is an alias
    /// to a specific column or table.
    fn parse_correlation_name_optional<'input>(&self, input: &'input str, tokens: &mut &[Token]) -> Result<Option<String>, StatementParseError<'input>> {
        if !tokens.consume_keyword(Keyword::As) {
            return Ok(None);
        }

        if is_end_of_statement(tokens) {
            return Err(StatementParseError::CorrelationNameUnexpectedEndOfFile);
        }

        if let Some(identifier) = tokens.consume_identifier_owned(input) {
            Ok(Some(identifier))
        } else {
            if let TokenKind::Keyword(keyword) = tokens[0].kind() {
                return Err(StatementParseError::CorrelationNameUnexpectedKeyword {
                    found: tokens[0].as_string(input),
                    keyword
                });
            }

            Err(StatementParseError::CorrelationNameUnexpectedToken {
                found: tokens[0].as_string(input),
                token_kind: tokens[0].kind()
            })
        }
    }

    /// Parse the `<insert columns and source>` section.
    fn parse_insert_columns_and_source<'input>(&self, input: &'input str, tokens: &mut &[Token]) -> Result<InsertColumnsAndSource, StatementParseError<'input>> {
        if is_end_of_statement(tokens) {
            return Err(StatementParseError::InsertColumnsAndSourceUnexpectedEndOfFileAtBeginning {
                found: input.slice_empty_end(),
            });
        }

        match tokens[0].kind() {
            TokenKind::Keyword(Keyword::Values) => {
                *tokens = &tokens[1..];

                Ok(InsertColumnsAndSource::FromConstructor {
                    insert_column_list: None,
                    constructor: self.parse_contextually_typed_table_value_constructor(input, tokens)?,
                })
            }

            TokenKind::Keyword(keyword) => Err(StatementParseError::InsertColumnsAndSourceUnexpectedKeyword {
                found: tokens[0].as_string(input),
                keyword,
            }),

            _ => Err(StatementParseError::InsertColumnsAndSourceUnexpectedToken {
                found: tokens[0].as_string(input),
                token_kind: tokens[0].kind(),
            }),
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
            Keyword::Insert => self.parse_statement_insert(input, &tokens),
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
            TokenKind::Keyword(Keyword::Database) => return self.parse_statement_create_database(input, tokens),
            TokenKind::Keyword(Keyword::Schema) => return self.parse_statement_create_schema(input, tokens),
            TokenKind::Keyword(Keyword::Table) => return self.parse_statement_create_table(input, tokens),
            TokenKind::Keyword(Keyword::View) => return self.parse_statement_create_view(input, tokens),

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
        return Err(StatementParseError::UnsupportedFeature {
            feature_name: "CREATE DATABASE",
            feature_description: "Creating new databases",
            found: tokens[0].as_string(input),
            token_kind: tokens[0].kind(),
        });
    }

    /// Parses the rest of the statement when the first two tokens were
    /// **`CREATE SCHEMA`**.
    fn parse_statement_create_schema<'input>(&self, input: &'input str, tokens: &[Token]) -> StatementResult<'input> {
        _ = input;
        _ = tokens;
        return Err(StatementParseError::UnsupportedFeature {
            feature_name: "CREATE SCHEMA",
            feature_description: "Creating new schemas",
            found: tokens[0].as_string(input),
            token_kind: tokens[0].kind(),
        });
    }

    /// Parses the rest of the statement when the first two tokens were
    /// **`CREATE TABLE`**.
    fn parse_statement_create_table<'input>(&self, input: &'input str, mut tokens: &[Token]) -> StatementResult<'input> {
        // Consume the `TABLE` keyword.
        tokens = &tokens[1..];

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
        return Err(StatementParseError::UnsupportedFeature {
            feature_name: "CREATE VIEW",
            feature_description: "Creating new views",
            found: tokens[0].as_string(input),
            token_kind: tokens[0].kind(),
        });
    }

    /// Parses the rest of the statement when the first token was the
    /// **`INSERT`** identifier keyword.
    fn parse_statement_insert<'input>(&self, input: &'input str, tokens: &[Token]) -> StatementResult<'input> {
        if is_end_of_statement(tokens) {
            return Err(StatementParseError::InsertStatementEndOfFile {
                found: input.slice_empty_end()
            });
        }

        match tokens[0].kind() {
            TokenKind::Keyword(Keyword::Into) => self.parse_statement_insert_into(input, &tokens[1..]),

            TokenKind::Keyword(keyword) => Err(StatementParseError::InsertStatementUnexpectedKeyword {
                found: tokens[0].as_string(input),
                keyword,
            }),

            _ => Err(StatementParseError::InsertStatementUnexpectedToken {
                found: tokens[0].as_string(input),
                token_kind: tokens[0].kind()
            })
        }
    }

    /// Parses the rest of the statement when the first tokens were the
    /// **`INSERT INTO`** identifier keywords.
    fn parse_statement_insert_into<'input>(&self, input: &'input str, mut tokens: &[Token]) -> StatementResult<'input> {
        if is_end_of_statement(tokens) {
            return Err(StatementParseError::InsertIntoStatementEndOfFile {
                found: &input[(input.len() - 1)..]
            });
        }

        if tokens[0].kind() != TokenKind::Identifier {
            return Err(StatementParseError::InsertIntoStatementUnexpectedToken {
                found: tokens[0].as_string(input),
                token_kind: tokens[0].kind()
            })
        }

        let table_name = tokens[0].as_string(input);
        tokens = &tokens[1..];

        let insert_columns_and_source = self.parse_insert_columns_and_source(input, &mut tokens)?;

        if !is_end_of_statement(tokens) {
            return Err(StatementParseError::InsertIntoStatementUnexpectedTrailingToken {
                found: tokens[0].as_string(input),
                token_kind: tokens[0].kind()
            });
        }

        let statement = InsertStatement {
            table_name: TableName {
                table_qualifier: table_name.to_owned()
            },
            insert_columns_and_source
        };

        Ok(SqlExecutableStatement::SqlDataStatement(
            SqlDataStatement::ChangeStatement(
                SqlDataChangeStatement::Insert(
                    statement
                )
            )
        ))
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

        let mut query_specification = QuerySpecification {
            select_list,
            set_quantifier,
            table_expression: None
        };

        if !is_end_of_statement(tokens) && tokens[0].kind() == TokenKind::Keyword(Keyword::From) {
            query_specification.table_expression = Some(self.parse_statement_select_table_expression(
                input, &mut tokens
            )?);
        }

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

    /// Parse the table expression (FROM, WHERE, GROUP BY and HAVING).
    /// ```text
    /// <table expression> ::=
    ///     <from clause>
    ///     [ <where clause> ]
    ///     [ <group by clause> ]
    ///     [ <having clause> ]
    /// ```
    fn parse_statement_select_table_expression<'input>(&self, input: &'input str, tokens: &mut &[Token]) -> Result<TableExpression, StatementParseError<'input>> {
        let table_expression = TableExpression {
            from_clause: self.parse_statement_select_table_expression_from_clause(input, tokens)?,
            group_by_clause: None,
            having_clause: None,
            where_clause: None,
        };

        if !is_end_of_statement(tokens) {
            if tokens[0].kind() == TokenKind::Keyword(Keyword::Having) {
                return Err(StatementParseError::UnsupportedFeature {
                    feature_name: "HAVING clause",
                    feature_description: "Filtering based on the GROUP BY clause",
                    found: tokens[0].as_string(input),
                    token_kind: tokens[0].kind()
                });
            }

            let mut toks = tokens.iter()
                .map(|tok| tok.kind());

            if toks.next() == Some(TokenKind::Keyword(Keyword::Group))
                 && toks.next() == Some(TokenKind::Keyword(Keyword::By)) {
                return Err(StatementParseError::UnsupportedFeature {
                    feature_name: "GROUP BY clause",
                    feature_description: "Grouping the SELECT statement columns",
                    found: tokens[0].as_string(input),
                    token_kind: tokens[0].kind(),
                });
            }

            if tokens[0].kind() == TokenKind::Keyword(Keyword::Where) {
                return Err(StatementParseError::UnsupportedFeature {
                    feature_name: "WHERE clause",
                    feature_description: "Filtering the SELECT statement columns",
                    found: tokens[0].as_string(input),
                    token_kind: tokens[0].kind(),
                });
            }
        }

        Ok(table_expression)
    }

    /// Parses a `<from clause>`
    fn parse_statement_select_table_expression_from_clause<'input>(&self, input: &'input str, tokens: &mut &[Token]) -> Result<FromClause, StatementParseError<'input>> {
        if is_end_of_statement(tokens) {
            return Err(StatementParseError::FromClauseUnexpectedEof);
        }

        if tokens[0].kind() != TokenKind::Keyword(Keyword::From) {
            return Err(StatementParseError::FromClauseUnexpectedToken {
                found: tokens[0].as_string(input),
                token_kind: tokens[0].kind()
            });
        }

        *tokens = &tokens[1..];

        let mut from_clause = FromClause {
            table_references: vec![
                self.parse_table_reference(input, tokens)?
            ],
        };

        loop {
            if is_end_of_statement(tokens) || tokens[0].kind() != TokenKind::Comma {
                break;
            }

            // Skip the comma token
            *tokens = &tokens[1..];

            from_clause.table_references.push(self.parse_table_reference(input, tokens)?);
        }

        Ok(from_clause)
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

        let mut sublist = Vec::new();

        loop {
            if is_end_of_statement(tokens) {
                if sublist.is_empty() {
                    return Err(StatementParseError::EofSelectList(input));
                }

                return Ok(SelectList::Sublist(sublist));
            }

            if !sublist.is_empty() && tokens[0].kind() != TokenKind::Comma {
                return Ok(SelectList::Sublist(sublist));
            }

            let tokens_saved = *tokens;

            let mut error = None;

            match self.parse_value_expression(input, tokens) {
                Ok(value_expression) => {
                    sublist.push(SelectSublist::DerivedColumn(DerivedColumn {
                        value_expression,
                        alias: None
                    }));
                    continue;
                }
                Err(e) => {
                    *tokens = tokens_saved;
                    error = Some(e);
                }
            }

            if let Some(error) = error {
                return Err(error);
            }

            return Err(StatementParseError::UnsupportedFeature {
                feature_name: "value expression",
                feature_description: "Diverse and/or complex value expressions",
                found: tokens[0].as_string(input),
                token_kind: tokens[0].kind(),
            });
        }
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

    /// Parses the list of table elements, before an opening parenthesis `(` was
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

    /// Parses a `<table reference>`
    ///
    /// ```text
    /// <table reference> ::=
    ///       <table primary>
    ///     | <joined table>
    /// ```
    fn parse_table_reference<'input>(&self, input: &'input str, tokens: &mut &[Token]) -> Result<TableReference, StatementParseError<'input>> {
        if is_end_of_statement(tokens) {
            return Err(StatementParseError::TableReferenceUnexpectedEndOfFile);
        }

        let first_token = tokens[0];

        *tokens = &tokens[1..];

        match first_token.kind() {
            TokenKind::Identifier => {
                let correlation_name = self.parse_correlation_name_optional(input, tokens)?;
                return Ok(TableReference::Primary(
                    TablePrimary {
                        kind: TablePrimaryKind::TableOrQueryName(first_token.as_string(input).to_owned()),
                        correlation_name,
                    }
                ))
            }

            TokenKind::Keyword(keyword) => Err(StatementParseError::TableReferenceUnexpectedKeyword {
                found: first_token.as_string(input),
                keyword
            }),

            _ => Err(StatementParseError::TableReferenceUnexpectedToken {
                found: first_token.as_string(input),
                token_kind: first_token.kind()
            })
        }
    }

    /// Parse a `<value expression>`.
    ///
    /// ```text
    /// <value expression> ::=
    //        <numeric value expression>
    //      | <string value expression>
    //      | <datetime value expression>
    //      | <interval value expression>
    //      | <boolean value expression>
    //      | <user-defined type value expression>
    //      | <row value expression>
    //      | <reference value expression>
    //      | <collection value expression>
    /// ```
    fn parse_value_expression<'input>(&self, input: &'input str, tokens: &mut &[Token]) -> Result<ValueExpression, StatementParseError<'input>> {
        if is_end_of_statement(tokens) {
            return Err(StatementParseError::ValueExpressionUnexpectedEndOfFile);
        }

        let first_token = tokens[0];
        *tokens = &tokens[1..];

        match first_token.kind() {
            TokenKind::UnsignedInteger(integer) => Ok(
                ValueExpression::Numeric(
                    NumericValueExpression::SimpleU64(integer)
                )
            ),

            _ => Err(StatementParseError::ValueExpressionUnexpectedToken {
                found: first_token.as_string(input),
                token_kind: first_token.kind(),
            })
        }
    }
}

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
#[derive(Copy, Clone, Debug, Error, PartialEq, EnumProperty, AsRefStr)]
pub enum StatementParseError<'input> {
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
    CorrelationNameUnexpectedEndOfFile,

    #[error("unexpected keyword: `{keyword}` (`{found}`): expected an identifier as the name of the correlation name (alias)")]
    #[strum(props(Hint="Did you forget to escape the identifier?"))]
    CorrelationNameUnexpectedKeyword {
        found: &'input str,
        keyword: Keyword,
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

    #[error("unexpected end-of-file, expected FROM clause")]
    FromClauseUnexpectedEof,

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

    #[error("unexpected keyword {keyword} (`{found}`), expected `VALUES` keyword")]
    #[strum(props(Help="Replace this with the `VALUES` keyword"))]
    InsertColumnsAndSourceUnexpectedKeyword {
        found: &'input str,
        keyword: Keyword,
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

    #[error("unexpected keyword {keyword} (`{found}`), expected `INTO` keyword")]
    #[strum(props(Hint="`INSERT` must be followed by the `INTO` keyword."))]
    #[strum(props(Help="Replace it with the `INTO` keyword: `INSERT INTO`"))]
    InsertStatementUnexpectedKeyword {
        found: &'input str,
        keyword: Keyword,
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

    #[error("unexpected token {token_kind}: `{found}`")]
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

    #[error("unexpected end-of-file, expected a table reference")]
    #[strum(props(Hint="Did you forget to add a table name?"))]
    TableReferenceUnexpectedEndOfFile,

    #[error("unexpected keyword: {keyword} (`{found}`), expected a table reference")]
    #[strum(props(Hint="Did you forget to escape the table or schema name?"))]
    #[strum(props(Help="`{keyword}` is reserved as a keyword"))]
    TableReferenceUnexpectedKeyword {
        found: &'input str,
        keyword: Keyword
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
    ValueExpressionUnexpectedEndOfFile,

    #[error("unexpected token: {token_kind} (`{found}`), expected a column name, value or expression")]
    ValueExpressionUnexpectedToken {
        found: &'input str,
        token_kind: TokenKind,
    },
}

impl<'input> StatementParseError<'input> {
    /// Get the property `found` if available. This describes where in the
    /// input the error was encountered.
    pub fn found(&self) -> Option<&'input str> {
        match self {
            StatementParseError::ContextuallyTypedRowValueConstructorUnexpectedEndOfFile { found, .. } => Some(found),
            StatementParseError::ContextuallyTypedRowValueConstructorUnexpectedEndOfFileExpectedCommaOrRightParen { found, .. } => Some(found),
            StatementParseError::ContextuallyTypedRowValueConstructorUnexpectedTokenExpectedCommaOrRightParen { found, .. } => Some(found),
            StatementParseError::ContextuallyTypedRowValueConstructorUnexpectedTokenExpectedLeftParen { found, .. } => Some(found),
            StatementParseError::CorrelationNameUnexpectedEndOfFile => None,
            StatementParseError::CorrelationNameUnexpectedKeyword { found, .. } => Some(found),
            StatementParseError::CorrelationNameUnexpectedToken { found, .. } => Some(found),
            StatementParseError::CreateStatementUnexpectedFollowUpToken { found, .. } => Some(found),
            StatementParseError::CreateTableStatementExpectedTableNameIdentifierUnexpectedEof { found, .. } => Some(found),
            StatementParseError::CreateTableStatementExpectedTableNameIdentifierUnexpectedKeyword { found, .. } => Some(found),
            StatementParseError::CreateTableStatementExpectedTableNameIdentifierUnexpectedToken { found, .. } => Some(found),
            StatementParseError::CreateTableUnexpectedEofAfterTableName => None,
            StatementParseError::CreateTableUnexpectedEofAfterTableNameAndLeftParen { found, .. } => Some(found),
            StatementParseError::CreateTableUnexpectedTokenAtEnd { found, .. } => Some(found),
            StatementParseError::EmptyInput => None,
            StatementParseError::EofCreateKeywordOnlyToken(found) => Some(found),
            StatementParseError::EofSelectKeywordOnlyToken(found) => Some(found),
            StatementParseError::EofSelectList(found) => Some(found),
            StatementParseError::FromClauseUnexpectedEof => None,
            StatementParseError::FromClauseUnexpectedToken { found, .. } => Some(found),
            StatementParseError::InsertColumnsAndSourceUnexpectedEndOfFileAtBeginning { found, .. } => Some(found),
            StatementParseError::InsertColumnsAndSourceUnexpectedKeyword { found, .. } => Some(found),
            StatementParseError::InsertColumnsAndSourceUnexpectedToken { found, .. } => Some(found),
            StatementParseError::InsertStatementEndOfFile { found, .. }  => Some(found),
            StatementParseError::InsertStatementUnexpectedKeyword { found, .. } => Some(found),
            StatementParseError::InsertStatementUnexpectedToken { found, .. }  => Some(found),
            StatementParseError::InsertIntoStatementEndOfFile { found, .. } => Some(found),
            StatementParseError::InsertIntoStatementUnexpectedToken { found, .. } => Some(found),
            StatementParseError::InsertIntoStatementUnexpectedTrailingToken { found, ..} => Some(found),
            StatementParseError::SelectStatementUnexpectedToken { found, .. } => Some(found),
            StatementParseError::StartNotAToken { found, .. } => Some(found),
            StatementParseError::StartUnknownKeyword { found, .. } => Some(found),
            StatementParseError::TableElementSingleExpectedKeywordAsDataType { found, .. } => Some(found),
            StatementParseError::TableElementSingleExpectedIdentifierAsColumnName { found, .. } => Some(found),
            StatementParseError::TableElementSingleExpectedIdentifierAsColumnNameButGotKeyword { found, .. } => Some(found),
            StatementParseError::TableElementSingleUnexpectedEndOfFileAfterColumnName => None,
            StatementParseError::TableElementSingleUnexpectedEndOfFileAtBeginning => None,
            StatementParseError::TableElementSingleUnknownDataTypeKeyword { found, .. } => Some(found),
            StatementParseError::TableElementsExpectedLeftParenthesis { found, .. } => Some(found),
            StatementParseError::TableElementsUnexpectedClosingParenthesis { found } => Some(found),
            StatementParseError::TableElementsUnexpectedCommaBeforeFirstColumn { found } => Some(found),
            StatementParseError::TableElementsUnexpectedEndOfFile => None,
            StatementParseError::TableElementsUnexpectedEndOfFileAfterComma { found } => Some(found),
            StatementParseError::TableElementsUnexpectedEndOfFileAtBeginning => None,
            StatementParseError::TableElementsUnexpectedSemicolon { found } => Some(found),
            StatementParseError::TableReferenceUnexpectedEndOfFile => None,
            StatementParseError::TableReferenceUnexpectedKeyword { found, .. } => Some(found),
            StatementParseError::TableReferenceUnexpectedToken { found, .. } => Some(found),
            StatementParseError::UnsupportedFeature { found, .. } => Some(found),
            StatementParseError::ValueExpressionUnexpectedEndOfFile => None,
            StatementParseError::ValueExpressionUnexpectedToken { found, .. } => Some(found),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::{
        Range,
        RangeFrom,
    };

    use super::*;
    use rstest::rstest;

    use pretty_assertions::assert_eq;

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
    #[case("SELECT 1", 1)]
    #[case("SELECT 1;", 1)]
    #[case("SELECT 693", 693)]
    fn parser_simple_select_number_literal_statement(#[case] input: &str, #[case] numeric_value: u64) {
        let derived_column = DerivedColumn {
            value_expression: ValueExpression::Numeric(NumericValueExpression::SimpleU64(numeric_value)),
            alias: None
        };

        let expected = SqlExecutableStatement::SqlDataStatement(
            SqlDataStatement::SelectStatement(
                QueryExpression{
                    body: QueryExpressionBody::SimpleTable(
                        SimpleTable::QuerySpecification(
                            QuerySpecification {
                                set_quantifier: SetQuantifier::All,
                                select_list: SelectList::Sublist(vec![
                                    SelectSublist::DerivedColumn(
                                        derived_column
                                    )
                                ]),
                                table_expression: None,
                            }
                        )
                    )
                }
            )
        );

        assert_eq!(Parser::new().parse_statement(input), Ok(expected));
    }

    fn parser_select_statement_erroneous_base<'input>(input: &'input str, expected: StatementParseError<'input>) {
        assert_eq!(Parser::new().parse_statement(input), Err(expected));
    }

    /// Can't select every column from every table unfortunately.
    ///
    /// However, it would be cool to implement something like this for some
    /// savages that want to search the whole schema / database.
    ///
    /// Syntactically, this can be implemented with a function, something like
    /// `ALL_TABLES_COMBINED()`?
    #[rstest]
    #[case("SELECT * FROM *", 14)]
    #[case("SELECT * FROM *;", 14)]
    #[case("SELECT * FROM    *", 17)]
    #[case("SELECT * FROM table1, *", 22)]
    #[case("SELECT * FROM  *,  ttceis8800;", 15)]
    #[test]
    fn parser_select_statement_erroneous_from_asterisk(#[case] input: &str, #[case] idx: usize) {
        parser_select_statement_erroneous_base(input, StatementParseError::TableReferenceUnexpectedToken {
            found: &input[idx..=idx],
            token_kind: TokenKind::Asterisk
        });
    }

    #[rstest]
    #[case(
        "INSERT INTO my_table VALUES (1)",
        "my_table",
        vec![
            vec![
                value_expression_simple_u64(1)
            ]
        ]
    )]
    #[case(
        "INSERT INTO cool_numbers VALUES (1, 2, 3, 5, 8, 13)",
        "cool_numbers",
        vec![
            vec![
                value_expression_simple_u64(1),
                value_expression_simple_u64(2),
                value_expression_simple_u64(3),
                value_expression_simple_u64(5),
                value_expression_simple_u64(8),
                value_expression_simple_u64(13),
            ]
        ]
    )]
    #[case(
        "INSERT INTO multiple_rows VALUES (2, 4), (1, 3), (4, 8)",
        "multiple_rows",
        vec![
            vec![
                value_expression_simple_u64(2),
                value_expression_simple_u64(4),
            ],
            vec![
                value_expression_simple_u64(1),
                value_expression_simple_u64(3),
            ],
            vec![
                value_expression_simple_u64(4),
                value_expression_simple_u64(8),
            ],
        ]
    )]
    fn parser_simple_insert_into_statement(#[case] input: &str, #[case] table_name: &str, #[case] rows: Vec<Vec<ValueExpression>>) {
        let result = Parser::new().parse_statement(input);

        let statement = InsertStatement {
            table_name: TableName {
                table_qualifier: table_name.to_owned(),
            },
            insert_columns_and_source: InsertColumnsAndSource::FromConstructor {
                insert_column_list: None,
                constructor: ContextuallyTypedTableValueConstructor {
                    values: rows.iter()
                        .map(|row| {
                            ContextuallyTypedRowValueExpression::ContextuallyTypedRowValueConstructor(
                                ContextuallyTypedRowValueConstructor {
                                    elements: row.iter()
                                        .map(|value| ContextuallyTypedRowValueConstructorElement::ValueExpression(value.clone()))
                                        .collect()
                                }
                            )
                        })
                        .collect()
                }
            }
        };

        let statement = SqlExecutableStatement::SqlDataStatement(
            SqlDataStatement::ChangeStatement(
                SqlDataChangeStatement::Insert(
                    statement
                )
            )
        );

        assert_eq!(result, Ok(statement));
    }

    #[rstest]
    #[case("SELECT * FROM my_table", &["my_table"], &[None])]
    #[case("SELECT * FROM ends_with_semicolon;", &["ends_with_semicolon"], &[None])]
    #[case("SELECT * FROM correlation AS corr;", &["correlation"], &[Some("corr")])]
    #[case("SELECT * FROM my_table AS tbl", &["my_table"], &[Some("tbl")])]
    #[case("SELECT * FROM table1, table2", &["table1", "table2"], &[None, None])]
    // This is an evaluation error, not a parse error.
    #[case("SELECT * FROM table1, table2 AS table1", &["table1", "table2"], &[None, Some("table1")])]
    #[case("SELECT * FROM table1 AS table3, table2", &["table1", "table2"], &[Some("table3"), None])]
    #[case("SELECT * FROM t1 AS t5, t2 as t78", &["t1", "t2"], &[Some("t5"), Some("t78")])]
    fn parser_simple_select_from_statement(#[case] input: &str, #[case] table_names: &[&str], #[case] correlation_names: &[Option<&str>]) {
        let result = Parser::new().parse_statement(input);
        if let Err(e) = &result {
            println!("Failed: {}", e);
        }

        let mut table_references = Vec::with_capacity(table_names.len());

        for (index, name) in table_names.iter().enumerate() {
            let correlation_name = correlation_names[index];
            table_references.push(
                TableReference::Primary(
                    TablePrimary {
                        kind: TablePrimaryKind::TableOrQueryName(name.to_string()),
                        correlation_name: correlation_name.map(|s| s.to_string())
                    }
                )
            );
        }

        let from_clause = FromClause {
            table_references
        };

        let body = QueryExpressionBody::SimpleTable(
            SimpleTable::QuerySpecification(
                QuerySpecification {
                    set_quantifier: SetQuantifier::All,
                    select_list: SelectList::Asterisk,
                    table_expression: Some(TableExpression {
                        from_clause,
                        where_clause: None,
                        group_by_clause: None,
                        having_clause: None
                    })
                }
            )
        );

        let expected = SqlExecutableStatement::SqlDataStatement(
            SqlDataStatement::SelectStatement(
                QueryExpression {
                    body
                }
            )
        );

        assert_eq!(result.unwrap(), expected);
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

    /// Create a simple u64 `<value expression>`
    const fn value_expression_simple_u64(value: u64) -> ValueExpression {
        ValueExpression::Numeric(
            NumericValueExpression::SimpleU64(value)
        )
    }

}
