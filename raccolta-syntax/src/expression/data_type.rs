// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::common::character_set::CharacterSetSpecification;

/// ```text
/// <character string type> ::=
///       CHARACTER [ <left paren> <length> <right paren> ]
///     | CHAR [ <left paren> <length> <right paren> ]
///     | CHARACTER VARYING <left paren> <length> <right paren>
///     | CHAR VARYING <left paren> <length> <right paren>
///     | VARCHAR <left paren> <length> <right paren>
///     | CHARACTER LARGE OBJECT [ <left paren> <large object length> <right paren> ]
///     | CHAR LARGE OBJECT [ <left paren> <large object length> <right paren> ]
///     | CLOB [ <left paren> <large object length> <right paren> ]
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum CharacterStringType {
    /// Non-Unicode, fixed
    Fixed {
        length: usize,
    },

    /// Non-Unicode, varying
    Varying {
        length: usize,
    },
}

/// ```text
/// <data type> ::=
///       <predefined type>
///     | <row type>
///     | <user-defined type>
///     | <reference type>
///     | <collection type>
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum DataType {
    Predefined(PredefinedType),
}

/// ```text
/// <national character string type> ::=
///       NATIONAL CHARACTER [ <left paren> <length> <right paren> ]
///     | NATIONAL CHAR [ <left paren> <length> <right paren> ]
///     | NCHAR [ <left paren> <length> <right paren> ]
///     | NATIONAL CHARACTER VARYING <left paren> <length> <right paren>
///     | NATIONAL CHAR VARYING <left paren> <length> <right paren>
///     | NCHAR VARYING <left paren> <length> <right paren>
///     | NATIONAL CHARACTER LARGE OBJECT [ <left paren> <large object length> <right paren> ]
///     | NCHAR LARGE OBJECT [ <left paren> <large object length> <right paren> ]
///     | NCLOB [ <left paren> <large object length> <right paren> ]
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum NationalCharacterStringType {
    /// Unicode, fixed
    Fixed {
        length: usize,
    },

    /// Unicode, varying
    Varying {
        length: usize,
    },
}

/// ```text
/// <numeric type> ::=
///       <exact numeric type>
///     | <approximate numeric type>
///
/// <exact numeric type> ::=
///       NUMERIC [ <left paren> <precision> [ <comma> <scale> ] <right paren> ]
///     | DECIMAL [ <left paren> <precision> [ <comma> <scale> ] <right paren> ]
///     | DEC [ <left paren> <precision> [ <comma> <scale> ] <right paren> ]
///     | INTEGER
///     | INT
///     | SMALLINT
///
/// <approximate numeric type> ::=
///       FLOAT [ <left paren> <precision> <right paren> ]
///     | REAL
///     | DOUBLE PRECISION
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum NumericType {
    Integer,

    Float {
        precision: Option<usize>
    },
    Real,
    DoublePrecision,
}

/// ```text
/// <predefined type> ::=
///       <character string type> [ CHARACTER SET <character set specification> ]
///     | <national character string type>
///     | <binary large object string type>
///     | <bit string type>
///     | <numeric type>
///     | <boolean type>
///     | <datetime type>
///     | <interval type>
///
#[derive(Clone, Debug, PartialEq)]
pub enum PredefinedType {
    /// `<character string type> [ CHARACTER SET <character set specification>`
    CharacterString {
        definition: CharacterStringType,
        character_set: Option<CharacterSetSpecification>,
    },

    /// `<national character string type>`
    NationalCharacterString(NationalCharacterStringType),

    // <--- `<binary large object>` ...

    // <--- `<bit string type>` ...

    /// `<numeric type>`
    Numeric(NumericType),

    /// `<boolean type>`
    Boolean,

    // <--- `<datetime type>` ...

    // <--- `<interval type>` ...
}
