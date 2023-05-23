// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::predicate::Predicate;

#[derive(Clone, Debug, PartialEq)]
pub enum BooleanExpression {
    Literal(bool),
    Predicate(Box<Predicate>),
}
