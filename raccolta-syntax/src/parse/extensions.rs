// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::{
    keyword::{
        ReservedWord,
        NonReservedWord,
    },
    Token,
    TokenKind,
};

pub(super) trait ParseArrayExtensions {
    /// Consumes an identifier and returns a borrowed [`str`].
    fn consume_identifier<'input>(self: &mut &Self, input: &'input str) -> Option<&'input str>;

    /// Consumes an identifier and returns an owned [`String`].
    fn consume_identifier_owned<'input>(self: &mut &Self, input: &'input str) -> Option<String> {
        self.consume_identifier(input).map(|str| str.to_owned())
    }

    /// Consumes a specific [`Keyword`].
    fn consume_reserved_word(self: &mut &Self, reserved_word: ReservedWord) -> bool;

    /// Consumes a specific [`Keyword`].
    fn consume_non_reserved_word(self: &mut &Self, non_reserved_word: NonReservedWord) -> bool;

    /// Checks if the following token is of type [`NonReservedWord`].
    fn is_non_reserved_word(&self) -> Option<NonReservedWord>;

    /// Checks if the following token is of type [`ReservedWord`].
    fn is_reserved_word(&self) -> Option<ReservedWord>;

    fn next(self: &mut &Self);
}

impl ParseArrayExtensions for [Token] {
    fn consume_identifier<'input>(self: &mut &Self, input: &'input str) -> Option<&'input str> {
        if let Some(token) = self.get(0) {
            if let TokenKind::Identifier = token.kind() {
                self.next();
                return Some(token.as_string(input));
            }
        }

        None
    }

    fn consume_reserved_word(self: &mut &Self, reserved_word: ReservedWord) -> bool {
        if self.is_reserved_word() == Some(reserved_word) {
            self.next();
            true
        } else {
            false
        }
    }

    fn consume_non_reserved_word(self: &mut &Self, non_reserved_word: NonReservedWord) -> bool {
        if self.is_non_reserved_word() == Some(non_reserved_word) {
            self.next();
            true
        } else {
            false
        }
    }

    fn is_reserved_word(&self) -> Option<ReservedWord> {
        self.get(0)
            .map(|token| token.kind().to_reserved_word())
            .flatten()
    }

    fn is_non_reserved_word(&self) -> Option<NonReservedWord> {
        self.get(0)
            .map(|token| token.kind().to_non_reserved_word())
            .flatten()
    }

    fn next(self: &mut &Self) {
        *self = &self[1..];
    }
}

pub(super) trait ParseStringExtensions {
    /// Get an empty slice, pointing at the end of the string, past the last
    /// character.
    ///
    /// This is useful for getting a lifetime to `self` and marking an EOF
    /// point.
    fn slice_empty_end<'a>(&'a self) -> &'a Self;
}

impl ParseStringExtensions for str {
    fn slice_empty_end<'a>(&'a self) -> &'a Self {
        &self[(self.len() - 1)..]
    }
}
