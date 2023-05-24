// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::expression::SimpleValueSpecification;

/// ```text
/// fetch_first_clause ::=
///     FETCH
///     ( FIRST | NEXT )
///     fetch_first_quantity?
///     ( ROW | ROWS )
///     ( ONLY | WITH TIES )
/// ```
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FetchFirstClause {
    pub quantity: FetchFirstQuantity,
}

/// fetch_first_quantity ::=
///       fetch_first_row_count
///     | fetch_first_percentage
///
///
/// fetch_first_row_count ::=
///     simple_value_specification
///
/// fetch_first_percentage ::=
///     simple_value_specification PERCENT
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FetchFirstQuantity {
    pub value: SimpleValueSpecification,
    pub is_percent: bool,
}
