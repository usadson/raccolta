// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

/// # References
/// ## 4.18.9 Known functional dependencies in the result of a `<having clause>`
/// Let *T1* be the table that is the operand of the `<having clause>`, let *SC*
/// be the `<search condition>` directly contained in the `<having clause>`, and
/// let *R* be the result of the `<having clause>`.
///
/// If *S* is a set of columns of *R* and the counterpart of *S* in *T1* is a
/// *BPK-set*, then *S* is a *BPK-set*. If the counterpart of *S* in *T1* is a
/// *BUC-set*, then *S* is a *BUC-set*.
///
/// Any known functional dependency in the `<query expression>`
/// ```sql
/// SELECT * FROM T1 WHERE SC
/// ```
/// is a *known functional dependency* in *R*.
#[derive(Clone, Debug, PartialEq)]
pub struct HavingClause {

}
