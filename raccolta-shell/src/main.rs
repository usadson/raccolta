// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::io::Write;

use raccolta_engine::EngineMessage;
use strum::EnumProperty;

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
                println!("Error({}): {e}", e.as_ref());

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
