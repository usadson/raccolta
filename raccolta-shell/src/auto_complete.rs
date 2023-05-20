// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::{
    cell::RefCell,
    rc::Rc,
};

use raccolta_engine::Engine;
use raccolta_syntax::{
    keyword::{
        NonReservedWord,
        ReservedWord,
    },
    Lexer,
    TokenKind, Token,
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

        if tokens.len() > 2
            && tokens[tokens.len() - 2].kind() == TokenKind::ReservedWord(ReservedWord::From)
            && tokens.last().unwrap().kind() == TokenKind::Identifier {
                return filter_strings(
                    self.engine.borrow()
                        .get_table_names()
                        .into_iter()
                        .map(|name| name.to_string()),
                    tokens.last().unwrap().as_string(input)
                );
            }

        Vec::new()
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
        _ = input;
        _ = highlighted_suggestion;
        Ok(None)
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
