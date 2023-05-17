// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::{io::Write, ops::Range};

use raccolta_engine::EngineMessage;
use strum::EnumProperty;

/// Let's say
/// haystack = "hello"
/// needle = "ll"
///
/// haystack_begin = 0
/// haystack_end = 5
///
/// needle_begin = 2
/// needle_end = 4
fn get_range_of_string_slice(haystack: &str, needle: &str) -> Option<Range<usize>> {
    if haystack.len() < needle.len() {
        return None;
    }

    let haystack_begin = haystack.as_ptr() as usize;
    let haystack_end = haystack.as_ptr() as usize + haystack.len();

    let needle_begin = needle.as_ptr() as usize;
    let needle_end = needle.as_ptr() as usize + needle.len();

    if haystack_begin > needle_begin || haystack_end < needle_end {
        return None;
    }

    let start = needle_begin - haystack_begin;
    Some(start..(start + needle.len()))
}

fn main() {
    println!("Raccolta Shell\n");

    let mut engine = raccolta_engine::Engine::new();

    let parser = raccolta_syntax::Parser::new();

    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        let line = std::io::stdin()
            .lines().next().unwrap()
            .unwrap();

        match parser.parse_statement(&line) {
            Ok(res) => {
                println!("{res:#?}");
                let result = engine.execute_statement(res);
                for message in result.messages {
                    match message {
                        EngineMessage::Error(err) => println!("Error: {err}"),
                        EngineMessage::Help(err) => println!("Info: {err}"),
                        EngineMessage::Hint(err) => println!("Help: {err}"),
                        EngineMessage::Informational(err) => println!("Info: {err}"),
                    }
                }

                println!("{} row(s)", result.row_count);
            }
            Err(e) => {
                println!("Error({}): {e}\n", e.as_ref());

                if let Some(found) = e.found() {
                    if let Some(range) = get_range_of_string_slice(&line, found) {
                        println!("Error occurred here: ");
                        println!("  {line}");
                        println!("  {}{}", " ".repeat(range.start), "^".repeat(range.end - range.start));
                    }
                }

                if let Some(hint) = e.get_str("Hint") {
                    println!("Hint: {hint}");
                }

                if let Some(help) = e.get_str("Help") {
                    println!("Help: {help}");
                }
            }
        }
    }
}
