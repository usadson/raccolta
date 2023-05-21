// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod auto_complete;
mod syntax_highlighted;

use std::{
    cell::RefCell,
    ops::Range,
    rc::Rc,
};

use raccolta_engine::EngineMessage;

use raccolta_syntax::{
    StatementParseError,
    Token,
    TokenKind,
};

use strum::EnumProperty;

use auto_complete::AutoCompleter;

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

    let engine = Rc::new(RefCell::new(raccolta_engine::Engine::new()));

    let parser = raccolta_syntax::Parser::new();

    loop {
        let line = inquire::Text::new(">")
            .with_autocomplete(AutoCompleter::new(Rc::clone(&engine)))
            .prompt();

        let line = match line {
            Ok(line) => line,
            Err(e) => {
                match e {
                    inquire::InquireError::OperationCanceled => continue,
                    inquire::InquireError::OperationInterrupted => return,
                    _ => panic!("Failed: {e:?}"),
                }
            }
        };

        let mut engine = engine.as_ref().borrow_mut();

        let (result, tokens) = parser.parse_statement_extended(&line);

        match result {
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

                if !result.column_names.is_empty() {
                    for column_name in result.column_names {
                        print!("{column_name}    ");
                    }
                    println!();

                    for row in result.row_iterator {
                        for value in row.values {
                            print!("{value}    ");
                        }
                        println!();
                    }
                }

                println!("{} row(s)", result.row_count);
            }
            Err(e) => {
                println!("Error({}): {e}\n", e.as_ref());

                print_error_findings(&line, &e, &tokens);

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

fn print_error_findings<'input>(
    input: &'input str,
    error: &StatementParseError<'input>,
    tokens: &[Token],
) {
    let mut matches = [
        error.found()
            .map(|found| {
                if let Some(range) = get_range_of_string_slice(input, found) {
                    let message = "error occurred here".to_string();
                    Some((range, message))
                } else {
                    None
                }
            })
            .flatten(),

        error.should_be_matching()
            .map(|should_be_matching| {
                if let Some(range) = get_range_of_string_slice(input, should_be_matching.found) {
                    let message = if let TokenKind::LeftParenthesis = should_be_matching.token_kind {
                        "match this opening parenthesis here".to_string()
                    } else {
                        format!("match this token ({:?}) here", should_be_matching.token_kind)
                    };
                    Some((range, message))
                } else {
                    None
                }
            })
            .flatten()
    ];

    matches.sort_by(|a, b| {
        if let Some(a) = a {
            if let Some(b) = b {
                a.0.start.cmp(&b.0.start)
            } else {
                std::cmp::Ordering::Greater
            }
        } else {
            std::cmp::Ordering::Equal
        }
    });

    let match_count = matches.iter().filter(|item| item.is_some()).count();

    // Nothing to show...
    if !matches.iter().any(|item| item.is_some()) {
        return;
    }

    print!("  ");
    syntax_highlighted::print_syntax_highlighted(input, &tokens);

    print!("  ");

    let mut last_point = 0;
    for match_item in &matches {
        let Some((range, message)) = match_item else { continue };

        assert!(last_point <= range.start);

        print!("{}^{}", " ".repeat(range.start - last_point), "~".repeat(range.end - range.start - 1));

        // If there is only one match, print the message on the same line.
        if match_count == 1 {
            println!(" {message}");
            return;
        }

        last_point = range.end;
    }

    println!();

    for (match_item_index, match_item) in matches.iter().enumerate().rev() {
        let Some((range, message)) = match_item else { continue };

        print!("  ");

        let mut start_point = 0;
        for match_item in matches[0..match_item_index].iter().rev() {
            let Some((range, _)) = match_item else { continue };
            let indent = " ".repeat(range.start - start_point);
            print!("{}|", indent);
            start_point = range.start + 1;
        }

        let indent = " ".repeat(range.start - start_point);
        println!("{}| {}", indent, message);
    }
}
