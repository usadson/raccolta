// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod insert;
mod table;

use std::{
    borrow::Cow,
    fmt::Debug,
};

use raccolta_syntax::{
    expression::data_type::{
        DataType,
        NumericType,
        PredefinedType,
    },
    schema::definition::table_definition::{
        TableDefinition,
        ColumnDefinition,
        TableElement,
    },
    statement::{
        insert_statement::{
            InsertStatement, InsertColumnsAndSource,
        },
        SqlDataStatement,
        SqlDataChangeStatement,
        SqlExecutableStatement,
        SqlSchemaDefinitionStatement,
        SqlSchemaStatement,
    },
};

use table::{
    EngineColumn,
    EngineColumnContainer,
    EngineColumnDescriptor,
    EngineTable,
};

/// The main entrypoint for the SQL/RDBMS engine for Raccolta, which executes
/// the parsed statements by `raccolta-syntax`.
#[derive(Debug)]
pub struct Engine {
    tables: Vec<EngineTable>,
}

impl Engine {
    /// Creates a new instance of the engine.
    pub fn new() -> Self {
        Self {
            tables: Vec::new(),
        }
    }

    fn create_value_container_for_column(&self, column: &ColumnDefinition) -> Option<EngineColumnContainer> {
        match column.data_type {
            DataType::Predefined(PredefinedType::Numeric(NumericType::Integer)) => {
                Some(EngineColumnContainer::Integers(Vec::new()))
            }
            _ => None
        }
    }

    /// Executes a parsed statement.
    pub fn execute_statement(&mut self, statement: SqlExecutableStatement) -> EngineResult {
        _ = statement;

        match statement {
            SqlExecutableStatement::Schema(statement) => self.execute_statement_schema(statement),
            SqlExecutableStatement::SqlDataStatement(statement) => self.execute_statement_data(statement),
        }
    }

    fn execute_statement_data(&mut self, statement: SqlDataStatement) -> EngineResult {
        match statement {
            SqlDataStatement::ChangeStatement(statement) => self.execute_statement_data_change(statement),
            _ => self.execute_unsupported_statement(),
        }
    }

    fn execute_statement_data_change(&mut self, statement: SqlDataChangeStatement) -> EngineResult {
        match statement {
            SqlDataChangeStatement::Insert(statement) => self.execute_statement_data_change_insert(statement),
        }
    }

    /// Executes the `INSERT INTO` statement.
    fn execute_statement_data_change_insert(&mut self, statement: InsertStatement) -> EngineResult {
        if self.tables.is_empty() {
            return EngineResult {
                messages: vec![
                    EngineMessage::Error("Failed to insert since there are no tables yet!".into())
                ],
                row_count: 0,
                row_iterator: Box::new(std::iter::empty())
            };
        }

        let table_ref = self.tables.iter_mut()
            .find(|table| table.name.eq_ignore_ascii_case(&statement.table_name.table_qualifier));

        let Some(table_ref) = table_ref else {
            return EngineResult {
                messages: vec![
                    EngineMessage::Error(format!("Unknown table named \"{}\"", statement.table_name.table_qualifier).into())
                ],
                row_count: 0,
                row_iterator: Box::new(std::iter::empty())
            };
        };

        match statement.insert_columns_and_source {
            InsertColumnsAndSource::FromConstructor { constructor, .. } => {
                insert::execute_from_contextually_typed_table_value_constructor(table_ref, constructor)
            }
        }
    }

    fn execute_statement_schema(&mut self, statement: SqlSchemaStatement) -> EngineResult {
        match statement {
            SqlSchemaStatement::Definition(statement) => self.execute_statement_schema_definition(statement),
            //_ => self.execute_unsupported_statement(),
        }
    }

    fn execute_statement_schema_definition(&mut self, statement: SqlSchemaDefinitionStatement) -> EngineResult {
        match statement {
            SqlSchemaDefinitionStatement::Table(statement) => self.execute_statement_schema_definition_table(statement),
            //_ => self.execute_unsupported_statement(),
        }
    }

    /// Executes the [`TableDefinition`] statement, which is colloquially known
    /// as the `CREATE TABLE` statement.
    fn execute_statement_schema_definition_table(&mut self, statement: TableDefinition) -> EngineResult {
        for table in &self.tables {
            if table.name.eq_ignore_ascii_case(&statement.table_name) {
                return EngineResult {
                    messages: vec![
                        EngineMessage::Error("A table with this name already exists".into()),
                        EngineMessage::Hint("Table names are case-insensitive, try to come up with a different name! :)".into())
                    ],
                    row_count: 0,
                    row_iterator: Box::new(std::iter::empty())
                };
            }
        }

        let mut columns = Vec::with_capacity(statement.elements.len());

        for element in statement.elements {
            let TableElement::ColumnDefinition(column) = element else {
                continue;
            };

            let Some(values) = self.create_value_container_for_column(&column) else {
                return EngineResult {
                    messages: vec![
                        EngineMessage::Error(format!("Failed to create column container for \"{}\"", column.column_name).into()),
                        EngineMessage::Error(format!("DataType is unsupported at the moment: {:#?}", column.data_type).into()),
                        EngineMessage::Help("You can create an issue at: https://github.com/usadson/raccolta/issues/new?template=bug_report.md".into()),
                    ],
                    row_count: 0,
                    row_iterator: Box::new(std::iter::empty()),
                }
            };

            columns.push(EngineColumn {
                descriptor: EngineColumnDescriptor {
                    name: column.column_name,
                    data_type: column.data_type
                },
                values,
            });
        }

        self.tables.push(EngineTable {
            name: statement.table_name,
            columns
        });

        EngineResult {
            messages: vec![
                EngineMessage::Informational("New table successfully created.".into()),
                EngineMessage::Informational(format!("DEBUG: Internal Representation of table: {:#?}", self.tables.last().unwrap()).into())
            ],
            row_count: 0,
            row_iterator: Box::new(std::iter::empty()),
        }
    }

    /// Returns a message describing that this statement is not yet supported
    /// at the moment.
    fn execute_unsupported_statement(&self) -> EngineResult {
        EngineResult {
            messages: vec![
                EngineMessage::Informational("Welcome to the Raccolta Engine!".into()),
                EngineMessage::Error("This statement is parsed, but unfortunately not yet supported for execution by the engine.".into()),
                EngineMessage::Help("You can create an issue at: https://github.com/usadson/raccolta/issues/new?template=bug_report.md".into()),
            ],
            row_count: 0,
            row_iterator: Box::new(std::iter::empty()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum EngineMessage {
    Error(Cow<'static, str>),
    Help(Cow<'static, str>),
    Hint(Cow<'static, str>),
    Informational(Cow<'static, str>),
}

/// The result of invoked statements.
pub struct EngineResult {
    /// Messages generated by the engine.
    pub messages: Vec<EngineMessage>,

    pub row_count: usize,
    pub row_iterator: Box<dyn Iterator<Item = EngineRow>>,
}

impl EngineResult {
    pub fn with_messages(messages: Vec<EngineMessage>) -> Self {
        Self {
            messages,
            row_count: 0,
            row_iterator: Box::new(std::iter::empty()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct EngineRow {

}
