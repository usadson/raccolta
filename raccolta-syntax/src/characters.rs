// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

//! This file contains convenience constants for common Unicode code points.
//!
//! “Unicode is a registered trademark of Unicode, Inc. in the United States
//! and other countries. This site is not in any way associated with or
//! endorsed or sponsored by Unicode, Inc. (aka The Unicode Consortium).”

/// Characters that are considered to be whitespace in SQL.
///
/// # References
/// * [Unicode Consortium - C0 Controls and Basic Latin](https://unicode.org/charts/PDF/U0000.pdf)
pub mod whitespace {
    /// All whitespace characters.
    pub const ALL: &[char] = &[
        CHARACTER_TABULATION,
        LINE_FEED,
        LINE_TABULATION,
        FORM_FEED,
        CARRIAGE_RETURN,
        SPACE,

        NO_BREAK_SPACE,
        EN_QUAD,
        EM_QUAD,
        EN_SPACE,
        EM_SPACE,
        THREE_PER_EM_SPACE,
        FOUR_PER_EM_SPACE,
        SIX_PER_EM_SPACE,
        FIGURE_SPACE,
        PUNCTUATION_SPACE,
        THIN_SPACE,
        HAIR_SPACE,
        ZERO_WIDTH_SPACE,
        ZERO_WIDTH_NON_JOINER,
        ZERO_WIDTH_JOINER,
        LEFT_TO_RIGHT_MARK,
        RIGHT_TO_LEFT_MARK,

        LINE_SEPARATOR,
        PARAGRAPH_SEPARATOR,

        IDEOGRAPHIC_SPACE,

        ZERO_WIDTH_NO_BREAK_SPACE,
    ];

    pub const CHARACTER_TABULATION: char = '\u{0009}';
    pub const LINE_FEED: char = '\u{000A}';
    pub const LINE_TABULATION: char = '\u{000B}';
    pub const FORM_FEED: char = '\u{000C}';
    pub const CARRIAGE_RETURN: char = '\u{000D}';
    pub const SPACE: char = '\u{0020}';

    pub const NO_BREAK_SPACE: char = '\u{00A0}';
    pub const EN_QUAD: char = '\u{2000}';
    pub const EM_QUAD: char = '\u{2001}';
    pub const EN_SPACE: char = '\u{2002}';
    pub const EM_SPACE: char = '\u{2003}';
    pub const THREE_PER_EM_SPACE: char = '\u{2004}';
    pub const FOUR_PER_EM_SPACE: char = '\u{2005}';
    // Where did FIVE_PER_EM_SPACE go though?
    pub const SIX_PER_EM_SPACE: char = '\u{2006}';
    pub const FIGURE_SPACE: char = '\u{2007}';
    pub const PUNCTUATION_SPACE: char = '\u{2008}';
    pub const THIN_SPACE: char = '\u{2009}';
    pub const HAIR_SPACE: char = '\u{200A}';
    pub const ZERO_WIDTH_SPACE: char = '\u{200B}';
    pub const ZERO_WIDTH_NON_JOINER: char = '\u{200C}';
    pub const ZERO_WIDTH_JOINER: char = '\u{200D}';
    pub const LEFT_TO_RIGHT_MARK: char = '\u{200E}';
    pub const RIGHT_TO_LEFT_MARK: char = '\u{200F}';

    pub const LINE_SEPARATOR: char = '\u{2028}';
    pub const PARAGRAPH_SEPARATOR: char = '\u{2029}';

    pub const IDEOGRAPHIC_SPACE: char = '\u{3000}';

    pub const ZERO_WIDTH_NO_BREAK_SPACE: char = '\u{FEFF}';
}

/// This trait contains extensions for SQL character definitions.
pub trait SqlCharacterExtensions {
    /// Is the character a whitespace according to the SQL specification.
    fn is_sql_whitespace(&self) -> bool;
}

impl SqlCharacterExtensions for char {
    fn is_sql_whitespace(&self) -> bool {
        whitespace::ALL.contains(self)
    }
}
