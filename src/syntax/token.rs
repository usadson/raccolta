// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use super::Keyword;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token {
    first_character_byte_idx: usize,
    last_character_byte_idx: usize,
    kind: TokenKind,
}

impl Token {
    /// Constructs a new [`Token`] instance.
    pub(super) fn new(first_character_byte_idx: usize, last_character_byte_idx: usize, kind: TokenKind) -> Self {
        Self { first_character_byte_idx, last_character_byte_idx, kind }
    }

    /// Returns the string representation as provided by the `input`.
    pub fn as_string<'a>(&self, input: &'a str) -> &'a str {
        &input[self.first_character_byte_idx..self.last_character_byte_idx]
    }

    /// Returns which kind of token this is.
    pub fn kind(&self) -> TokenKind {
        self.kind
    }
}

/// A token is a lexical constituent.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenKind {
    /// The '*' character.
    Asterisk,

    /// The ',' character
    Comma,

    /// The '=' character
    EqualsSign,

    /// The '.' character
    FullStop,

    /// The '>' character.
    GreaterThanSign,

    Keyword(Keyword),

    //// The '(' character.
    LeftParenthesis,

    /// The '<' character.
    LessThanSign,

    Identifier,

    /// The '%' character.
    PercentageSign,

    /// The '+' character.
    PlusSign,

    /// The ')' character.
    RightParenthesis,

    /// The ';' character.
    Semicolon,

    /// A string literal, e.g. 'Hello, world!'.
    StringLiteral,
}
