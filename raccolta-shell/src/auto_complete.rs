// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::{
    cell::RefCell,
    rc::Rc,
};

use raccolta_engine::Engine;
use raccolta_syntax::{
    expression::{
        ColumnReference,
        query_expression::{
            QueryExpressionBody,
            SimpleTable,
        },
        query_specification::{
            SelectList,
            SelectSublist
        },
    },
    keyword::{
        NonReservedWord,
        ReservedWord,
    },
    Lexer,
    statement::{
        SqlDataStatement,
        SqlExecutableStatement,
    },
    Token,
    TokenKind,
};
use strum::IntoEnumIterator;

#[derive(Clone, Debug)]
pub struct AutoCompleter {
    engine: Rc<RefCell<Engine>>,
}

impl AutoCompleter {
    pub fn new(engine: Rc<RefCell<Engine>>) -> Self {
        Self { engine }
    }

    fn complete_select(&self, input: &str, tokens: &[Token]) -> Vec<String> {
        if tokens.len() == 2 && tokens[1].kind() == TokenKind::Asterisk {
            return vec!["FROM".into()];
        }

        if tokens.len() == 3 && tokens[1].kind() == TokenKind::Asterisk {
            let last_word = tokens[2].as_string(input);
            if !last_word.eq_ignore_ascii_case("FROM") {
                return filter_keywords([ReservedWord::From].iter(), tokens[2].as_string(input));
            }
        }

        if tokens.last().unwrap().kind() == TokenKind::ReservedWord(ReservedWord::From)
            && input.chars().last().unwrap().is_whitespace() {
            return self.engine.borrow()
                .get_table_names()
                .into_iter()
                .map(|name| name.to_string())
                .collect();
        }

        if let Some(suggestions) = self.complete_select_order_by(input, tokens) {
            return suggestions;
        }

        if tokens.len() > 2 {
            let identifier = tokens.last().unwrap().as_string(input);

            if let TokenKind::ReservedWord(reserved_word) = tokens[tokens.len() - 2].kind() {
                match reserved_word {
                    ReservedWord::By => {
                        if tokens.len() > 3 && tokens[tokens.len() - 3].kind() == TokenKind::ReservedWord(ReservedWord::Order) {
                            let order_token = tokens[tokens.len() - 3];

                            let statement = raccolta_syntax::Parser::new()
                                .parse_statement(&input[0..order_token.first_character_byte_idx]);
                            match statement {
                                Ok(statement) => return self.suggest_column_names(statement, identifier),
                                Err(_) => (),
                            }
                        }
                    }

                    ReservedWord::From => {
                        return filter_strings(
                            self.engine.borrow()
                                .get_table_names()
                                .into_iter()
                                .map(|name| name.to_string()),
                            identifier
                        );
                    }

                    ReservedWord::Order => {
                        return filter_strings(["BY".into()].into_iter(), identifier);
                    }

                    _ => ()
                }
            }
        }

        Vec::new()
    }

    /// Complete the `ORDER BY` clause of the `SELECT` statement.
    fn complete_select_order_by(&self, input: &str, tokens: &[Token]) -> Option<Vec<String>> {
        // `ORDER` can only be followed by `BY` in `SELECT` statements.
        // There is `ORDER FULL BY`, but this is not applicable to statements.
        if tokens.last().map(|token|token.kind()) == Some(TokenKind::ReservedWord(ReservedWord::Order)) {
            return Some(vec!["BY".into()]);
        }

        let last_token = tokens.last().to_owned();
        let second_last_token = tokens.iter().nth_back(1).to_owned();

        let last_token_reserved_word = last_token.map(|token| token.as_reserved_word()).flatten();
        let second_last_token_reserved_word = second_last_token.map(|token| token.as_reserved_word()).flatten();

        if second_last_token_reserved_word == Some(ReservedWord::Order)
                && last_token_reserved_word == Some(ReservedWord::By)
                && input.chars().last().unwrap().is_whitespace() {
            let result = raccolta_syntax::Parser::new()
                .parse_statement(&input[0..second_last_token.unwrap().first_character_byte_idx]);
            if let Ok(statement) = result {
                if let Some(column_names) = self.get_column_names(statement) {
                    return Some(column_names.collect());
                }
            }
        }

        None
    }

    /// Get the tokens that can occur at the start of the statement.
    fn get_starting_tokens(&self) -> Vec<String> {
        vec![
            "ALTER".into(),
            "CREATE".into(),
            "DECLARE".into(),
            "DROP".into(),
            "INSERT".into(),
            "RENAME".into(),
            "SELECT".into(),
            "SET".into(),
            "TRUNCATE".into(),
            "UPDATE".into(),
            "WITH".into(),
        ]
    }

    fn get_column_names(&self, statement: SqlExecutableStatement) -> Option<Box<dyn Iterator<Item = String>>> {
        if let SqlExecutableStatement::SqlDataStatement(SqlDataStatement::SelectStatement(query_expression)) = statement {
            if let QueryExpressionBody::SimpleTable(SimpleTable::QuerySpecification(query_specification)) = query_expression.body {
                if let SelectList::Sublist(sublist) = query_specification.select_list {
                    let iterator = sublist
                        .into_iter()
                        .map(|sublist| {
                            match sublist {
                                SelectSublist::DerivedColumn(column) => match column.value_expression {
                                    raccolta_syntax::expression::ValueExpression::ColumnReference(column) => {
                                        match column {
                                            ColumnReference::BasicIdentifierChain(chain) => Some(chain.join(".")),
                                            _ => None,
                                        }
                                    }
                                    _ => None,
                                }
                            }
                        })
                        .flatten();
                    return Some(Box::new(iterator));
                }
            }
        }

        None
    }

    fn suggest_column_names(&self, statement: SqlExecutableStatement, identifier: &str) -> Vec<String> {
        match self.get_column_names(statement) {
            Some(iterator) => filter_strings(iterator, identifier),
            None => Vec::new()
        }
    }
}

fn filter_all_keywords(word: &str) -> Vec<String> {
    filter_keywords(
        ReservedWord::iter()
            .map(|keyword| keyword.to_string())
            .chain(NonReservedWord::iter()
                .map(|keyword| keyword.to_string())),
        word
    )
}

fn filter_keywords<KeywordType>(iterator: impl Iterator<Item = KeywordType>, word: &str) -> Vec<String>
        where KeywordType: ToString {
    filter_strings(iterator
        .map(|keyword| keyword.to_string()), word)
}

fn filter_strings(iterator: impl Iterator<Item = String>, word: &str) -> Vec<String> {
            let word = word.to_ascii_lowercase();

    iterator
        .map(|keyword| keyword.to_string())
        .filter(|keyword| {
            if keyword.len() < word.len() {
                return false;
            }
            keyword[0..word.len()].eq_ignore_ascii_case(&word)
        })
        .collect()
}

fn starts_with_ignore_case(haystack: &str, needle: &str) -> bool {
    if haystack.len() < needle.len() {
        return false;
    }
    haystack[0..needle.len()].eq_ignore_ascii_case(needle)
}

impl inquire::Autocomplete for AutoCompleter {
    fn get_completion(
            &mut self,
            input: &str,
            highlighted_suggestion: Option<String>,
        ) -> Result<Option<String>, inquire::CustomUserError> {
        // If there wasn't a suggestion to begin with, nothing is to be done.
        // We might suggest something that requires a heavy computation, but
        // at the moment, there isn't such a thing, since [`get_suggestions`] is
        // quite fast.
        let Some(highlighted_suggestion) = highlighted_suggestion else {
            return Ok(None);
        };

        // If the user hasn't typed anything yet (i.e. it's completing the first
        // token), we can just suggest this suggestion.
        // Add a space after it for rapid typing ^_^
        if input.trim().is_empty() {
            return Ok(Some(format!("{} ", highlighted_suggestion)));
        }

        // Otherwise, we need to find out what word is

        // If there isn't a token to be completed (e.g. not SEL -> SELECT),
        // append the suggestion to the input.
        if input.ends_with(' ') {
            return Ok(Some(format!("{}{} ", input, highlighted_suggestion)));
        }


        // Complete the input with the highlighted suggestion:
        // E.g. input = "SELECT * FR" highlighted_suggestion = "FROM"
        // -> "SELECT * FROM"

        let input = input.rfind(' ')
            .map(|last_space_idx| &input[..last_space_idx + 1])
            .unwrap_or("");
        Ok(Some(format!("{input}{highlighted_suggestion} ")))
    }

    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, inquire::CustomUserError> {
        let tokens: Vec<_> = Lexer::new(input).collect();

        if tokens.is_empty() {
            return Ok(self.get_starting_tokens());
        }

        match tokens[0].kind() {
            TokenKind::ReservedWord(ReservedWord::Create) => {
                let words = ["DATABASE", "SCHEMA", "TABLE", "VIEW"];
                if tokens.len() == 1{
                    return Ok(words.iter().map(|word| word.to_string()).collect());
                }
                if tokens.len() == 2 {
                    let second_token = tokens[1].as_string(input);
                    return Ok(filter_keywords(words.iter(), second_token));
                }
            }

            TokenKind::ReservedWord(ReservedWord::Insert) => {
                if tokens.len() == 1 {
                    return Ok(vec!["INTO".into()]);
                }
                if tokens.len() == 2 {
                    let second_token = tokens[1].as_string(input);
                    if second_token.len() < 4 && "INTO"[0..second_token.len()].eq_ignore_ascii_case(second_token) {
                        return Ok(vec!["INTO".into()]);
                    }
                }
            },

            TokenKind::ReservedWord(ReservedWord::Select) => return Ok(self.complete_select(input, &tokens)),

            TokenKind::Identifier => {
                if tokens.len() == 1 {
                    let word = tokens[0].as_string(input);
                    let mut keywords: Vec<_> = self.get_starting_tokens()
                        .into_iter()
                        .filter(|keyword| starts_with_ignore_case(keyword, word))
                        .collect();

                    let mut other_words: Vec<_> = filter_all_keywords(word)
                        .into_iter()
                        .filter(|word| {
                            !keywords.contains(word)
                        })
                        .collect();
                    other_words.sort();

                    keywords.extend(other_words);

                    return Ok(keywords);
                }
            }

            _ => ()
        }

        if let Some(last) = tokens.last() {
            match last.kind() {
                TokenKind::Identifier | TokenKind::ReservedWord(..) | TokenKind::NonReservedWord(..) => {
                    return Ok(filter_all_keywords(last.as_string(input)));
                }
                _ => ()
            }
        }

        Ok(Vec::new())
    }
}
