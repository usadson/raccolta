// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod clause;
mod common;
mod expression;
mod keyword;
mod parse;
mod schema;
mod set_function;
mod statement;
mod token;

pub use keyword::Keyword;
pub use token::{Token, TokenKind};

pub use parse::{
    Parser,
    StatementParseError,
};

use strum::IntoEnumIterator;

#[derive(Clone, Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    character_byte_idx: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            character_byte_idx: 0
        }
    }

    /// Consumes a token. If it returns `None`, there are no characters left.
    pub fn consume_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        if self.character_byte_idx >= self.input.len() {
            return None;
        }

        let first_character_byte_idx = self.character_byte_idx;

        let Some(first_character) = self.input[first_character_byte_idx..].chars().nth(0) else {
            return None;
        };

        self.next_character();

        let kind = match first_character {
            '*' => TokenKind::Asterisk,
            ',' => TokenKind::Comma,
            '=' => TokenKind::EqualsSign,
            '.' => TokenKind::FullStop,
            '>' => TokenKind::GreaterThanSign,
            '(' => TokenKind::LeftParenthesis,
            '<' => TokenKind::LessThanSign,
            '%' => TokenKind::PercentageSign,
            '+' => TokenKind::PlusSign,
            ')' => TokenKind::RightParenthesis,
            ';' => TokenKind::Semicolon,

            'a'..='z' | 'A'..='Z' => {
                while self.is_current_character_identifier_body_character() {
                    self.next_character();
                }

                let str = &self.input[first_character_byte_idx..self.character_byte_idx];

                match Keyword::iter().find(|keyword| keyword.as_ref().eq_ignore_ascii_case(str)) {
                    Some(keyword) => TokenKind::Keyword(keyword),
                    None => TokenKind::Identifier
                }
            }

            '0'..='9' => {
                let mut value = ((first_character as u8) - b'0') as u64;

                while self.is_current_character_digit() {
                    value *= 10;
                    value += ((self.current_character().unwrap() as u8) - b'0') as u64;

                    self.next_character();
                }

                TokenKind::UnsignedInteger(value)
            }

            _ => {
                println!("Unexpected character: '{first_character}' dec={} hex=0x{:X}",
                    first_character as usize, first_character as usize);
                todo!()
            }
        };

        Some(Token::new(
            first_character_byte_idx,
            self.character_byte_idx,
            kind
        ))
    }

    /// Gets the current character pointed to by the [`index`].
    fn current_character(&self) -> Option<char> {
        self.input[self.character_byte_idx..].chars().nth(0)
    }

    /// Check if the current character is a `<digit>`.
    fn is_current_character_digit(&self) -> bool {
        let Some(character) = self.current_character() else {
            return false;
        };

        matches!(character, '0'..='9')
    }

    /// Check if the current character is an `<identifier body character>`.
    fn is_current_character_identifier_body_character(&self) -> bool {
        let Some(character) = self.current_character() else {
            return false;
        };

        matches!(character, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_')
    }

    /// Checks if the character pointed to by [`index`] is whitespace.
    fn is_current_character_whitespace(&self) -> bool {
        let Some(character) = self.current_character() else {
            return false;
        };

        Self::is_whitespace(character)
    }

    /// Checks if the character is whitespace.
    fn is_whitespace(character: char) -> bool {
        matches!(character, ' ' | '\n' | '\r' | '\t')
    }

    /// Advances the index to the next character.
    fn next_character(&mut self) {
        if let Some(next_byte_offset) = self.input[self.character_byte_idx..].char_indices().nth(1) {
            self.character_byte_idx += next_byte_offset.0;
        } else {
            self.character_byte_idx += 1;
        }
    }

    /// Skips the whitespace in the `input`.
    fn skip_whitespace(&mut self) {
        while self.is_current_character_whitespace() {
            self.next_character();
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.consume_token()
    }
}

#[cfg(test)]
mod tests {
    use crate::syntax::*;

    use super::Lexer;

    use pretty_assertions::{
        assert_eq
    };


    #[test]
    fn lexer_simple_select_query() {
        let input = "SELECT * FROM blog_posts;";

        let tokens: Vec<_> = Lexer::new(input)
            .collect();

        assert_eq!(tokens, vec![
            Token::new(0, 6, TokenKind::Keyword(Keyword::Select)),

            Token::new(7, 8, TokenKind::Asterisk),

            Token::new(9, 13, TokenKind::Keyword(Keyword::From)),

            Token::new(14, 24, TokenKind::Identifier),

            Token::new(24, 25, TokenKind::Semicolon),
        ]);
    }

}
