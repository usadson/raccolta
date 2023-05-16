// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::io::Write;

use strum::EnumProperty;

fn main() {
    println!("Raccolta Shell\n");

    let parser = raccolta_syntax::Parser::new();
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        let line = std::io::stdin()
            .lines().next().unwrap()
            .unwrap();

        match parser.parse_statement(&line) {
            Ok(res) => println!("{res:#?}"),
            Err(e) => {
                println!("Error({}): {e}", e.as_ref());

                if let Some(hint) = e.get_str("Hint") {
                    println!("HINT: {hint}");
                }

                if let Some(help) = e.get_str("Help") {
                    println!("HELP: {help}");
                }
            }
        }
    }
}
