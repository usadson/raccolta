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
    pub origin: FetchFirstClauseOrigin,
}

/// Clarifies where this [`FetchFirstClause`] came from. Since many vendor
/// extensions such as `LIMIT` and `TOP` predate `FETCH FIRST`, it is good
/// practice to support those as well.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FetchFirstClauseOrigin {
    /// Originated from standard SQL `FETCH FIRST`
    FetchFirst,

    /// Originated from MySQL vendor extension `LIMIT`
    Limit,

    /// Originated from T-SQL vendor extension `TOP`
    Top,
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
