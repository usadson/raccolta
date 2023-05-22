// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

pub mod clause;
pub mod common;
pub mod expression;
pub mod keyword;
pub mod parse;
pub mod predicate;
pub mod schema;
pub mod set_function;
pub mod statement;
pub mod token;

use keyword::{ReservedWord, NonReservedWord};
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

        let Some(first_character) = self.input[first_character_byte_idx..].chars().next() else {
            return None;
        };

        self.next_character();

        let kind = match first_character {
            '!' => TokenKind::ExclamationMark,
            '"' => todo!(),
            '#' => todo!("invalid token"),
            '$' => TokenKind::DollarSign,
            '%' => TokenKind::PercentageSign,
            '&' => TokenKind::Ampersand,
            '\'' => loop {
                let Some(next_character) = self.current_character() else {
                    break TokenKind::IllegalToken;
                };

                if next_character == '\'' {
                    let last_character_byte_idx = self.character_byte_idx;
                    // This is a valid string, but the first character points to
                    // ' token, which is incorrect.
                    self.next_character();

                    break TokenKind::StringLiteral {
                        first_character_byte_idx: first_character_byte_idx + 1,
                        last_character_byte_idx
                    };
                }

                self.next_character();
            }
            '(' => TokenKind::LeftParenthesis,
            ')' => TokenKind::RightParenthesis,
            '*' => TokenKind::Asterisk,
            '+' => TokenKind::PlusSign,
            ',' => TokenKind::Comma,
            '-' => TokenKind::MinusSign,
            '.' => TokenKind::FullStop,
            '/' => TokenKind::Solidus,

            '0'..='9' => {
                let mut value = ((first_character as u8) - b'0') as u64;

                while self.is_current_character_digit() {
                    value *= 10;
                    value += ((self.current_character().unwrap() as u8) - b'0') as u64;

                    self.next_character();
                }

                TokenKind::UnsignedInteger(value)
            }

            ':' => TokenKind::Colon,
            ';' => TokenKind::Semicolon,
            '<' => TokenKind::LessThanOperator,
            '=' => TokenKind::EqualsSign,
            '>' => TokenKind::GreaterThanOperator,
            '?' => TokenKind::Question,
            '@' => TokenKind::AtSign,

            'a'..='z' | 'A'..='Z' => {
                while self.is_current_character_identifier_body_character() {
                    self.next_character();
                }

                let str = &self.input[first_character_byte_idx..self.character_byte_idx];

                match ReservedWord::iter().find(|reserved_word| reserved_word.as_ref().eq_ignore_ascii_case(str)) {
                    Some(reserved_word) => TokenKind::ReservedWord(reserved_word),

                    None => match NonReservedWord::iter().find(|non_reserved_word| non_reserved_word.as_ref().eq_ignore_ascii_case(str)) {
                        Some(non_reserved_word) => TokenKind::NonReservedWord(non_reserved_word),
                        None => TokenKind::Identifier
                    }
                }
            }

            '[' => todo!(),
            '\\' => TokenKind::ReverseSolidus,
            ']' => todo!(),
            '^' => TokenKind::Circumflex,
            '_' => todo!(),
            '`' => todo!(),
            '{' => TokenKind::LeftBrace,
            '|' => TokenKind::VerticalBar,
            '}' => TokenKind::RightBrace,
            '~' => todo!(),

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
        self.input[self.character_byte_idx..].chars().next()
    }

    /// Check if the current character is a `<digit>`.
    fn is_current_character_digit(&self) -> bool {
        let Some(character) = self.current_character() else {
            return false;
        };

        character.is_ascii_digit()
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
    use super::*;

    use super::Lexer;

    use pretty_assertions::{
        assert_eq
    };


    #[test]
    fn lexer_simple_select_query() {
        let input = "SELECT * FROM blog_posts WHERE title = 'Hello, World!';";

        let tokens: Vec<_> = Lexer::new(input)
            .collect();

        assert_eq!(tokens, vec![
            Token::new(0, 6, TokenKind::ReservedWord(ReservedWord::Select)),

            Token::new(7, 8, TokenKind::Asterisk),

            Token::new(9, 13, TokenKind::ReservedWord(ReservedWord::From)),

            Token::new(14, 24, TokenKind::Identifier),

            Token::new(25, 30, TokenKind::ReservedWord(ReservedWord::Where)),

            Token::new(31, 36, TokenKind::Identifier),

            Token::new(37, 38, TokenKind::EqualsSign),

            Token::new(39, 54, TokenKind::StringLiteral {
                first_character_byte_idx: 40,
                last_character_byte_idx: 53
            }),

            Token::new(54, 55, TokenKind::Semicolon),
        ]);
    }

}
