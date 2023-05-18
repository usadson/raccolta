// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use raccolta_engine::{Engine, EngineRowColumnValue, EngineRow};
use raccolta_syntax::Parser;
use rstest::rstest;

#[rstest]
#[case("t", "SELECT * FROM t")]
#[case("z0", "SELECT * FROM z0")]
#[case("a_", "SELECT * FROM a_")]
#[case("my_table_1", "SELECT * FROM my_table_1")]
#[case("MY_TABLE_1", "SELECT * FROM my_table_1")]
#[case("my_table_1", "SELECT * FROM MY_TABLE_1")]
#[case("MY_TABLE_1", "SELECT * FROM MY_TABLE_1")]
#[case("my_table_2", "SELECT DISTINCT * FROM my_table_2")]
#[case("MY_TABLE_2", "SELECT DISTINCT * FROM my_table_2")]
#[case("my_table_2", "SELECT DISTINCT * FROM MY_TABLE_2")]
#[case("MY_TABLE_2", "SELECT DISTINCT * FROM MY_TABLE_2")]
fn select_empty(#[case] table_name: &str, #[case] query: &str) {
    let parser = Parser::new();
    let mut engine = Engine::new();

    let create_statement = parser.parse_statement(&format!("CREATE TABLE {table_name} (num INT);"))
        .expect("failed to generate `CREATE TABLE` statement");

    _ = engine.execute_statement(create_statement);

    let query_statement = parser.parse_statement(query)
        .expect("failed to parse statement");

    let result = engine.execute_statement(query_statement);
    assert_eq!(result.row_count, 0);
    assert_eq!(result.column_names, vec!["num"]);

    let mut row_iterator = result.row_iterator;
    assert_eq!(row_iterator.next(), None);
}

#[rstest]
#[case(
    "CREATE TABLE my_favorite_numbers (num INT);",
    "INSERT INTO my_favorite_numbers VALUES (6)",
    "SELECT * FROM my_favorite_numbers",
    vec![
        EngineRow { values: vec![EngineRowColumnValue::I32(6)] },
    ]
)]
#[case(
    "CREATE TABLE my_favorite_numbers (num INT);",
    "INSERT INTO my_favorite_numbers VALUES (5), (8), (6616), (0), (52262365)",
    "SELECT * FROM my_favorite_numbers",
    vec![
        EngineRow { values: vec![EngineRowColumnValue::I32(5)] },
        EngineRow { values: vec![EngineRowColumnValue::I32(8)] },
        EngineRow { values: vec![EngineRowColumnValue::I32(6616)] },
        EngineRow { values: vec![EngineRowColumnValue::I32(0)] },
        EngineRow { values: vec![EngineRowColumnValue::I32(52262365)] },
    ]
)]
fn select_some(#[case] create_statement: &str, #[case] insert_statement: &str, #[case] query: &str, #[case] rows: Vec<EngineRow>) {
    use pretty_assertions::assert_eq;

    let parser = Parser::new();
    let mut engine = Engine::new();

    _ = engine.execute_statement(parser.parse_statement(create_statement).expect("failed to parse create_statement"));
    _ = engine.execute_statement(parser.parse_statement(insert_statement).expect("failed to parse insert_statement"));

    let query_statement = parser.parse_statement(query)
        .expect("failed to parse statement");

    let result = engine.execute_statement(query_statement);
    assert_eq!(result.row_count, rows.len());
    assert_eq!(result.column_names, vec!["num"]);

    assert_eq!(result.row_iterator.collect::<Vec<EngineRow>>(), rows);
}
