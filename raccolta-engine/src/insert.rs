// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use raccolta_syntax::expression::{
    data_type::{
        DataType,
        NumericType,
        PredefinedType,
    },
    NumericValueExpression,
    row_value_expression::ContextuallyTypedRowValueExpression,
    row_value_constructor::ContextuallyTypedRowValueConstructorElement,
    table_value_constructor::ContextuallyTypedTableValueConstructor,
    ValueExpression,
};

use crate::{table::EngineTable, EngineResult, EngineMessage};



/// Executes the `INSERT INTO` statement, after the table was found to
/// insert into.
pub fn execute_from_contextually_typed_table_value_constructor(
    table: &mut EngineTable, constructor: ContextuallyTypedTableValueConstructor
) -> EngineResult {
    match execute_from_contextually_typed_table_value_constructor_impl(table, constructor) {
        Ok(result) => result,
        Err(result) => result,
    }
}

fn execute_from_contextually_typed_table_value_constructor_impl(
    table: &mut EngineTable, constructor: ContextuallyTypedTableValueConstructor
) -> Result<EngineResult, EngineResult> {
    validate_table_with_constructor(table, &constructor)?;

    let row_count = constructor.values.len();

    for row in constructor.values {
        match row {
            ContextuallyTypedRowValueExpression::ContextuallyTypedRowValueConstructor(constructor) => {
                for (column_idx, column) in constructor.elements.into_iter().enumerate() {
                    table.columns[column_idx].append(column)?;
                }
            }
        }
    }

    Ok(EngineResult::with_messages(vec![
        EngineMessage::Informational(
            format!(
                "Inserted {} row(s) into table \"{}\", now totaling {} row(s)",
                row_count,
                table.name,
                table.columns[0].values.len()
            ).into()
        ),
        EngineMessage::Informational(
            format!("Data: {:#?}", table).into()
        )
    ]))
}

fn is_column_value_trivially_convertible_to(column_value: &ContextuallyTypedRowValueConstructorElement, data_type: &DataType) -> bool {
    match column_value {
        ContextuallyTypedRowValueConstructorElement::ValueExpression(expression) => match expression {
            ValueExpression::Numeric(numeric_expression) => match numeric_expression {
                NumericValueExpression::SimpleU64(..) => match data_type {
                    DataType::Predefined(PredefinedType::Numeric(NumericType::Integer)) => true,
                    _ => false,
                }
            }
            _ => todo!()
        }
    }
}

/// Validate the table with the given contextually typed table value
/// constructor. Returns [`None`] if no error was found, otherwise [`Some`].
fn validate_table_with_constructor(table: &EngineTable, constructor: &ContextuallyTypedTableValueConstructor) -> Result<(), EngineResult> {
    for (row_idx, row) in constructor.values.iter().enumerate() {
        match row {
            ContextuallyTypedRowValueExpression::ContextuallyTypedRowValueConstructor(constructor) => {
                if table.columns.len() != constructor.elements.len() {
                    return Err(EngineResult::with_messages(vec![
                        EngineMessage::Error(
                            format!(
                                "Table \"{}\" contains {} columns, but {} were provided in row {}",
                                table.name,
                                table.columns.len(),
                                constructor.elements.len(),
                                row_idx + 1
                            ).into()
                        )
                    ]));
                }

                for (column_idx, column_value) in constructor.elements.iter().enumerate() {
                    let descriptor = &table.columns[column_idx].descriptor;
                    if !is_column_value_trivially_convertible_to(column_value, &descriptor.data_type) {
                        return Err(EngineResult::with_messages(vec![
                            EngineMessage::Error("Value is not trivially convertible to type".into()),
                            EngineMessage::Hint(format!("Value: {:#?}", column_value).into()),
                            EngineMessage::Hint(format!("Data Type: {:#?}", descriptor.data_type).into()),
                            EngineMessage::Hint(
                                format!(
                                    "Found in row {} at column {}",
                                    row_idx,
                                    column_idx
                                ).into()
                            )
                        ]));
                    }
                }
            }
        }
    }

    Ok(())
}
