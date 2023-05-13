// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use super::Keyword;

pub struct Token {
    pub(super) first_character_byte_idx: usize,
    pub(super) last_character_byte_idx: usize,
    pub(super) kind: TokenKind,
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
