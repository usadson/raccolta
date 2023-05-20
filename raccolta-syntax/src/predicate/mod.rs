// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

//! This module contains the structures for predicates in SQL.

pub mod comparison_predicate;

use self::comparison_predicate::ComparisonPredicate;

/// The predicate defines a condition that can be evaluated to a boolean value.
#[derive(Clone, Debug, PartialEq)]
pub enum Predicate {
    /// A predicate that compares two values using a specified operator.
    Comparison(ComparisonPredicate),
}
