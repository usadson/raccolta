// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

//! This module contains iterator extensions for sorting engine rows.

use raccolta_syntax::clause::order_by_clause::OrderingSpecification;

use crate::EngineRow;

/// A sorting element is a translated step from `ORDER BY` clauses that specify
/// in which way and which order to sort the table. This is part of
/// [`EngineSortingElement`], which contain all steps.
pub struct EngineSortingElement {
    pub column_index: usize,
    pub ordering_specification: OrderingSpecification,
}

/// Sorting methods are translated steps from `ORDER BY` clauses that specify
/// in which way and which order to sort the table. This struct can be empty,
///  which means that no sorting is requested.
pub type EngineSortingMethod = Vec<EngineSortingElement>;

/// This trait adds access to the [`EngineRowSortIterator`] to iterators
/// which have an item of type [`EngineRow`].
pub trait EngineRowSortIteratorExtensionTrait
        where Self: Sized + Iterator<Item = EngineRow> + 'static {
    /// Sort (`ORDER BY` in SQL terms) the table using the given
    /// [`EngineSortingMethod`]. This can be empty, which means that no sorting
    /// is requested.
    fn apply_order_by(
        self,
        sorting_method: EngineSortingMethod
    ) -> Box<dyn Iterator<Item = EngineRow>> {
        if sorting_method.is_empty() {
            return Box::new(self);
        }

        if sorting_method.len() == 1 {
            return Box::new(self.engine_sort_rows(
                sorting_method[0].column_index,
                sorting_method[0].ordering_specification,
            ));
        }

        let mut iter: Box<dyn Iterator<Item = EngineRow>> = Box::new(self);

        for element in sorting_method {
            iter = Box::new(iter.engine_sort_rows(
                element.column_index,
                element.ordering_specification,
            ));
        }

        iter
    }

    /// Sort the rows based on a column index and an ordering specification.
    fn engine_sort_rows(
        self,
        column_index: usize,
        ordering_specification: OrderingSpecification
    ) -> std::vec::IntoIter<EngineRow>;
}

impl<Iter> EngineRowSortIteratorExtensionTrait for Iter
        where Iter: Sized + Iterator<Item = EngineRow> + 'static {
    fn engine_sort_rows(
        self,
        column_index: usize,
        ordering_specification: OrderingSpecification
    ) -> std::vec::IntoIter<EngineRow> {
        let mut rows: Vec<_> = self.collect();
        rows.sort_by(|a, b| {
            a.values[column_index].compare_ordering(&b.values[column_index], ordering_specification)
        });
        rows.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::EngineRowColumnValue;

    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        vec![],
        OrderingSpecification::Ascending,
        &[],
    )]
    #[case(
        vec![],
        OrderingSpecification::Descending,
        &[],
    )]
    #[case(
        vec![
            EngineRow {
                values: Vec::new(),
            }
        ],
        OrderingSpecification::Ascending,
        &[
            EngineRow {
                values: Vec::new(),
            }
        ],
    )]
    #[case(
        vec![
            EngineRow {
                values: Vec::new(),
            }
        ],
        OrderingSpecification::Descending,
        &[
            EngineRow {
                values: Vec::new(),
            }
        ],
    )]
    #[case(
        vec![
            EngineRow { values: vec![EngineRowColumnValue::I32(1)] }
        ],
        OrderingSpecification::Ascending,
        &[
            EngineRow { values: vec![EngineRowColumnValue::I32(1)] }
        ],
    )]
    fn test_engine_sort_rows_single_column_integers(
        #[case] inputs: Vec<EngineRow>,
        #[case] ordering_specification: OrderingSpecification,
        #[case] expected: &[EngineRow],
    ) {
        let output: Vec<_> = inputs
            .into_iter()
            .engine_sort_rows(0, ordering_specification)
            .collect();

        assert_eq!(&output, expected);
    }
}
