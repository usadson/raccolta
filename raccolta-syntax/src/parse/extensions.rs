// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::{
    Keyword,
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
    fn consume_keyword(self: &mut &Self, keyword: Keyword) -> bool;

    /// Checks if the following token is of type [`Keyword`].
    fn is_keyword(&self) -> Option<Keyword>;

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

    fn consume_keyword(self: &mut &Self, keyword: Keyword) -> bool {
        if self.is_keyword() == Some(keyword) {
            self.next();
            true
        } else {
            false
        }
    }

    fn is_keyword(&self) -> Option<Keyword> {
        self.get(0)
            .map(|token| token.kind().to_keyword())
            .flatten()
    }

    fn next(self: &mut &Self) {
        *self = &self[1..];
    }
}
