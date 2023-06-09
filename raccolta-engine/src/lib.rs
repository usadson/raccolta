// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

//! This crate contains the SQL/RDBMS engine for Raccolta, which executes the
//! parsed statements by `raccolta-syntax`.

mod insert;
mod select;
mod sorting;
mod table;

use std::{
    borrow::Cow,
    collections::HashMap,
    fmt::{
        Debug,
        Display,
    },
    sync::{
        Arc,
        RwLock,
    },
};

use bitvec::prelude::*;
use unicase::UniCase;

use raccolta_syntax::{
    clause::{
        fetch_first_clause::FetchFirstClause,
        order_by_clause::{
            OrderByClause,
            OrderingSpecification,
        },
    },
    expression::{
        data_type::{
            DataType,
            NumericType,
            PredefinedType, CharacterStringType,
        },
        QueryExpression,
        QuerySpecification,
        table_reference::{
            TablePrimaryKind,
            TableReference,
        },
    },
    schema::definition::table_definition::{
        TableDefinition,
        ColumnDefinition,
        TableElement,
    },
    statement::{
        insert_statement::{
            InsertStatement,
            InsertColumnsAndSource,
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
    tables: HashMap<UniCase<Arc<str>>, Arc<RwLock<EngineTable>>>,
}

impl Engine {
    /// Creates a new instance of the engine.
    pub fn new() -> Self {
        Self {
            tables: HashMap::new(),
        }
    }

    pub fn new_debug() -> Self {
        let table_name = Arc::from("People");
        let mut tables = HashMap::new();
        tables.insert(
            UniCase::from(Arc::clone(&table_name)),
            Arc::new(RwLock::new(EngineTable {
                name: table_name,
                columns: vec![
                    EngineColumn {
                        descriptor: EngineColumnDescriptor {
                            name: "Name".into(),
                            data_type: DataType::Predefined(PredefinedType::CharacterString { definition: CharacterStringType::Varying { length: 50 }, character_set: None }),
                        },
                        values: EngineColumnContainer::StringsVarying {
                            values: vec![
                                "Barrack Obama".into(),
                                "Michael Jackson".into(),
                                "Albert Einstein".into(),
                                "Chris Lattner".into(),
                                "Graydon Hoare".into(),
                            ],
                            maximum_length: 50
                        }
                    },
                    EngineColumn {
                        descriptor: EngineColumnDescriptor {
                            name: "HasDied".into(),
                            data_type: DataType::Predefined(PredefinedType::Boolean),
                        },
                        values: EngineColumnContainer::Booleans(bitvec![
                            0,
                            1,
                            1,
                            0,
                            0,
                        ])
                    }
                ]
            }))
        );
        Self {
            tables
        }
    }

    fn create_value_container_for_column(&self, column: &ColumnDefinition) -> Option<EngineColumnContainer> {
        match &column.data_type {
            DataType::Predefined(PredefinedType::Boolean) => {
                Some(EngineColumnContainer::Booleans(BitVec::new()))
            }
            DataType::Predefined(PredefinedType::Numeric(NumericType::Integer)) => {
                Some(EngineColumnContainer::Integers(Vec::new()))
            }
            DataType::Predefined(PredefinedType::CharacterString { definition, .. }) => {
                match definition {
                    CharacterStringType::Varying { length } => Some(
                        EngineColumnContainer::StringsVarying {
                            values: Vec::new(),
                            maximum_length: *length
                        }
                    ),
                    _ => None
                }
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
            SqlDataStatement::SelectStatement(statement) => self.execute_statement_select(statement),

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
            return EngineResult::with_messages(vec![
                EngineMessage::Error("Failed to insert since there are no tables yet!".into())
            ]);
        }

        let table_name = UniCase::new(Arc::from(statement.table_name.table_qualifier.as_ref()));
        let table_ref = self.tables.get(&table_name);

        let Some(table_ref) = table_ref else {
            return EngineResult::with_messages(vec![
                EngineMessage::Error(format!("Unknown table named \"{}\"", statement.table_name.table_qualifier).into())
            ]);
        };

        match statement.insert_columns_and_source {
            InsertColumnsAndSource::FromConstructor { constructor, .. } => {
                insert::execute_from_contextually_typed_table_value_constructor(table_ref.clone(), constructor)
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
        if self.tables.contains_key(&UniCase::new(Arc::from(statement.table_name.as_ref()))) {
            return EngineResult::with_messages(vec![
                EngineMessage::Error("A table with this name already exists".into()),
                EngineMessage::Hint("Table names are case-insensitive, try to come up with a different name! :)".into())
            ]);
        }

        let mut columns = Vec::with_capacity(statement.elements.len());

        for element in statement.elements {
            #[allow(irrefutable_let_patterns)]
            let TableElement::ColumnDefinition(column) = element else {
                continue;
            };

            let Some(values) = self.create_value_container_for_column(&column) else {
                return EngineResult::with_messages(vec![
                    EngineMessage::Error(format!("Failed to create column container for \"{}\"", column.column_name).into()),
                    EngineMessage::Error(format!("DataType is unsupported at the moment: {:#?}", column.data_type).into()),
                    EngineMessage::Help("You can create an issue at: https://github.com/usadson/raccolta/issues/new?template=bug_report.md".into()),
                ]);
            };

            columns.push(EngineColumn {
                descriptor: EngineColumnDescriptor {
                    name: column.column_name,
                    data_type: column.data_type
                },
                values,
            });
        }

        let table_name = Arc::from(statement.table_name.as_ref());
        let table = EngineTable {
            name: Arc::clone(&table_name),
            columns
        };

        self.tables.insert(
            UniCase::new(table_name),
            Arc::new(RwLock::new(table))
        );

        EngineResult::with_messages(vec![
            EngineMessage::Informational("New table successfully created.".into())
        ])
    }

    fn execute_statement_select(&mut self, statement: QueryExpression) -> EngineResult {
        match statement.body {
            raccolta_syntax::expression::query_expression::QueryExpressionBody::SimpleTable(simple_table) => {
                match simple_table {
                    raccolta_syntax::expression::query_expression::SimpleTable::QuerySpecification(query_specification) => {
                        self.execute_statement_select_simple_table_query_specification(query_specification, statement.order_by, statement.fetch)
                    }
                }
            }
            _ => self.execute_unsupported_statement()
        }
    }

    fn execute_statement_select_simple_table_query_specification(
        &self,
        query_specification: QuerySpecification,
        order_by_clause: Option<OrderByClause>,
        fetch_first_clause: Option<FetchFirstClause>,
    ) -> EngineResult {
        let Some(table_expression) = &query_specification.table_expression else {
            return self.execute_unsupported_statement();
        };

        if table_expression.from_clause.table_references.len() != 1 {
            return self.execute_unsupported_statement();
        }

        let table_name = match &table_expression.from_clause.table_references[0] {
            TableReference::Primary(primary) => match &primary.kind {
                TablePrimaryKind::TableOrQueryName(name) => name
            }
        };

        let Some(table_ref) = self.tables.get(&UniCase::new(Arc::from(table_name.clone()))) else {
            return EngineResult::with_messages(vec![
                EngineMessage::Error(format!("Unknown table named \"{}\"", table_name).into())
            ]);
        };

        select::execute(query_specification, table_ref.clone(), order_by_clause, fetch_first_clause)
    }

    /// Returns a message describing that this statement is not yet supported
    /// at the moment.
    fn execute_unsupported_statement(&self) -> EngineResult {
        EngineResult::with_messages(vec![
            EngineMessage::Informational("Welcome to the Raccolta Engine!".into()),
            EngineMessage::Error("This statement is parsed, but unfortunately not yet supported for execution by the engine.".into()),
            EngineMessage::Help("You can create an issue at: https://github.com/usadson/raccolta/issues/new?template=bug_report.md".into()),
        ])
    }

    /// Get the names of all tables.
    pub fn get_table_names(&self) -> Vec<Arc<str>> {
        self.tables.values()
            .map(|table| {
                if let Ok(table) = table.read() {
                    Some(Arc::clone(&table.name))
                } else {
                    None
                }
            })
            .flatten()
            .collect()
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

    pub column_names: Vec<String>,
    pub row_count: usize,
    pub row_iterator: Box<dyn Iterator<Item = EngineRow>>,
}

impl EngineResult {
    pub fn with_messages(messages: Vec<EngineMessage>) -> Self {
        Self {
            messages,
            column_names: Vec::new(),
            row_count: 0,
            row_iterator: Box::new(std::iter::empty()),
        }
    }
}

/// A result row.
#[derive(Clone, Debug, PartialEq)]
pub struct EngineRow {
    pub values: Vec<EngineRowColumnValue>,
}

/// The value of a column in a result row.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum EngineRowColumnValue {
    Bool(bool),
    I32(i32),
    String(String),
}

impl EngineRowColumnValue {
    fn compare_ordering(
        &self,
        other: &EngineRowColumnValue,
        ordering_specification: OrderingSpecification
    ) -> std::cmp::Ordering {
        match &self {
            Self::Bool(self_value) => match &other {
                Self::Bool(other_value) => {
                    let ordering = self_value.cmp(other_value);
                    match ordering_specification {
                        OrderingSpecification::Ascending => ordering,
                        OrderingSpecification::Descending => ordering.reverse(),
                    }
                }
                _ => std::cmp::Ordering::Equal,
            }
            Self::I32(self_value) => match &other {
                Self::I32(other_value) => {
                    let ordering = self_value.cmp(other_value);
                    match ordering_specification {
                        OrderingSpecification::Ascending => ordering,
                        OrderingSpecification::Descending => ordering.reverse(),
                    }
                }
                _ => std::cmp::Ordering::Equal,
            }
            Self::String(self_value) => match &other {
                Self::String(other_value) => {
                    let ordering = self_value.cmp(other_value);
                    match ordering_specification {
                        OrderingSpecification::Ascending => ordering,
                        OrderingSpecification::Descending => ordering.reverse(),
                    }
                }
                _ => std::cmp::Ordering::Equal,
            }
        }
    }
}

impl Display for EngineRowColumnValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Bool(b) => Display::fmt(&b, f),
            Self::I32(i) => Display::fmt(&i, f),
            Self::String(s) => Display::fmt(&s, f),
        }
    }
}
