// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crossterm::{
    execute,
    style::{
        Attribute,
        Color,
        Print,
        ResetColor,
        SetAttribute,
        SetForegroundColor,
    },
};
use raccolta_syntax::{
    Token,
    TokenKind,
};

pub fn print_syntax_highlighted(input: &str, tokens: &[Token]) {
    let mut index = 0;
    for token in tokens {
        print!("{}", &input[index..token.first_character_byte_idx]);
        index = token.first_character_byte_idx;

        match token.kind() {
            TokenKind::Identifier => {
                _ = execute!(
                    std::io::stdout(),
                    SetForegroundColor(Color::Green),
                    Print(token.as_string(input)),
                    ResetColor
                );
                index = token.last_character_byte_idx;
            }

            TokenKind::NonReservedWord(..) => {
                _ = execute!(
                    std::io::stdout(),
                    SetForegroundColor(Color::Blue),
                    SetAttribute(Attribute::Bold),
                    Print(token.as_string(input)),
                    SetAttribute(Attribute::Reset),
                    ResetColor
                );
                index = token.last_character_byte_idx;
            }

            TokenKind::ReservedWord(..) => {
                _ = execute!(
                    std::io::stdout(),
                    SetForegroundColor(Color::Magenta),
                    SetAttribute(Attribute::Bold),
                    Print(token.as_string(input)),
                    SetAttribute(Attribute::Reset),

                    ResetColor
                );
                index = token.last_character_byte_idx;
            }

            _ => ()
        }
    }

    println!("{}", &input[index..]);
}
