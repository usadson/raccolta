// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::keyword::{
    NonReservedWord,
    ReservedWord, VendorReservedWord,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token {
    pub first_character_byte_idx: usize,
    pub last_character_byte_idx: usize,
    kind: TokenKind,
}

impl Token {
    /// Constructs a new [`Token`] instance.
    pub(super) fn new(first_character_byte_idx: usize, last_character_byte_idx: usize, kind: TokenKind) -> Self {
        Self { first_character_byte_idx, last_character_byte_idx, kind }
    }

    pub fn as_non_reserved_word(&self) -> Option<NonReservedWord> {
        if let TokenKind::NonReservedWord(word) = self.kind {
            Some(word)
        } else {
            None
        }
    }

    pub fn as_reserved_word(&self) -> Option<ReservedWord> {
        if let TokenKind::ReservedWord(word) = self.kind {
            Some(word)
        } else {
            None
        }
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
#[derive(strum::Display)]
pub enum TokenKind {
    /// The '&' token
    Ampersand,

    /// The '*' token
    Asterisk,

    /// The '@' token
    AtSign,

    /// The '^' token
    Circumflex,

    /// The `:` token
    Colon,

    /// The ',' character
    Comma,

    /// The `||` token
    ConcatenationOperator,

    /// The `$` token
    DollarSign,

    /// The `::` token
    DoubleColon,

    /// The `..` token
    DoublePeriod,

    /// The '=' character
    EqualsSign,

    /// The `! token
    ExclamationMark,

    /// The '.' character
    FullStop,

    /// The `<` token
    GreaterThanOperator,

    /// The `>=` token
    GreaterThanOrEqualsOperator,

    Identifier,

    /// Illegal portion
    IllegalToken,

    /// The `{` token
    LeftBrace,

    /// The `{-` token
    LeftBraceMinus,

    //// The '(' character.
    LeftParenthesis,

    /// The `<` token
    LessThanOperator,

    /// The `<=` token
    LessThanOrEqualsOperator,

    /// The `-` token
    MinusSign,

    /// The `=>` token
    NamedArgumentAssignmentOperator,

    /// A [`NonReservedWord`].
    NonReservedWord(NonReservedWord),

    /// The `<>` token
    NotEqualsOperator,

    /// The '%' character.
    PercentageSign,

    /// The '+' character.
    PlusSign,

    /// The `?` token
    Question,

    /// A [`ReservedWord`].
    ReservedWord(ReservedWord),

    /// The `\` token
    ReverseSolidus,

    /// The `->` token
    RightArrow,

    /// The `}` token
    RightBrace,

    /// The `-}` token
    RightMinusBrace,

    /// The ')' character.
    RightParenthesis,

    /// The ';' character.
    Semicolon,

    /// The `/` token
    Solidus,

    /// A string literal, e.g. 'Hello, world!'.
    StringLiteral {
        first_character_byte_idx: usize,
        last_character_byte_idx: usize,
    },

    /// An unsigned integer [`u64`].
    UnsignedInteger(u64),

    VendorReservedWord(VendorReservedWord),

    /// The `|` token
    VerticalBar,
}

impl TokenKind {
    /// Converts this token kind to a [`NonReservedWord`] if applicable.
    pub const fn to_non_reserved_word(&self) -> Option<NonReservedWord> {
        if let TokenKind::NonReservedWord(non_reserved_word) = self {
            Some(*non_reserved_word)
        } else {
            None
        }
    }

    /// Converts this token kind to a [`ReservedWord`] if applicable.
    pub const fn to_reserved_word(&self) -> Option<ReservedWord> {
        if let TokenKind::ReservedWord(reserved_word) = self {
            Some(*reserved_word)
        } else {
            None
        }
    }
}
