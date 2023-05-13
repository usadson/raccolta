// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.


/// The `<set quantifier>` specifies a quantification method of e.g. an
/// `SELECT`-query. This can be either **`DISTINCT`** or **`ALL`**.
///
/// **`ALL`** is implied if no `<set quantifier>` is provided.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum SetQuantifier {
    /// The **`ALL`** quantifier doesn't process the rows in the query result,
    /// and just returns all the rows, even if there might be redundant
    /// duplicate rows (as is removed by the **`DISTINCT`** quantifier).
    ///
    /// This is the default implicit value, but can be explicitly mentioned via
    /// the **`ALL`** keyword.
    #[default]
    All,

    /// The distinct quantifier removes redundant duplicate rows in the query
    /// result.
    Distinct,
}
