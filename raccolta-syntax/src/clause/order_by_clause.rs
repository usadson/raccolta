// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::expression::ColumnReference;

/// This specifies in which order the rows should be sorted in, ASCending or
/// DESCending. Ascending sorts from lowest to highest, descending from highest
/// to lowest.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum OrderingSpecification {
    /// Sort from lowest to highest.
    Ascending,

    /// Sort from highest to lowest.
    Descending,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct OrderByClause {
    pub sort_specification_list: Vec<SortSpecification>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SortSpecification {
    /// TODO: This isn't completely correct, as it should be a
    /// `<value expression>`, with a lot of restrictions.
    pub sort_key: ColumnReference,

    /// Specifies in which order the rows should be sorted in.
    pub ordering_specification: Option<OrderingSpecification>
}
