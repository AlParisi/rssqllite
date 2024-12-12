use std::collections::HashMap;
use sqlparser::ast::{DataType, Expr, Statement};

use sqlparser::ast::Value as SqlValue;
use sqlparser::dialect::MySqlDialect;
use sqlparser::parser::Parser;
use crate::btree::btreenode::Row;
use crate::table::table::Table;

pub fn parse_and_execute(query: &str) -> Result<(), String> {

    let dialect = MySqlDialect {};
    let statements = Parser::parse_sql(&dialect, query)
        .map_err(|e| format!("Failed to parse SQL: {}", e))?;

    for statement in statements {
        match statement {
            /**
            * CREATE TABLE foo (PersonID int,LastName varchar(255),FirstName varchar(255));
            */
            Statement::CreateTable (create) => {
                let mut map: HashMap<String, DataType> = HashMap::new();
                for column in create.columns {
                    println!("{} {}", column.name, column.data_type);
                    map.insert(column.name.to_string(), column.data_type);

                }
                Table::new(create.name.to_string(), map);
                println!("Creating table '{}'", create.name);
            }
            /**
            * INSERT INTO foo (PersonID, LastName, FirstName) VALUES (1, 'al','rossi');
            */
            Statement::Insert( insert) => {
                //don't like it, to find another solution
                let tb_name= insert.table_name.to_string();
                let mut table  = Table::get_table(tb_name);
                let mut splitted: String = insert.source.iter().map(|v| v.body.to_string()).collect();
                let mut val: Vec<DataType> = Vec::new();
                println!("splitted : {:?}", parse_values(splitted.as_str()));
                table.insert_row(Row::new(val));
            }
            _ => {
                println!("Unsupported statement: {:?}", statement);
            }
        }
        //still to fix this!
        if query.eq_ignore_ascii_case("select"){
            let mut table  = Table::get_table("foo".to_string());
            table.print_rows();
        }
    }

    Ok(())
}

pub fn parse_values(values_str: &str) -> Result<Vec<SqlValue>, String> {
    // Remove "VALUES" if present and trim parentheses
    let cleaned_str = values_str
        .trim_start_matches("VALUES")
        .trim()
        .trim_matches(|c| c == '(' || c == ')');

    // Split by comma, but handle quoted strings carefully
    let parsed_values = parse_sql_values(cleaned_str)?;

    Ok(parsed_values)
}

fn parse_sql_values(input: &str) -> Result<Vec<SqlValue>, String> {
    let mut values = Vec::new();
    let mut current_value = String::new();
    let mut in_string = false;
    let mut escape = false;
    let mut current_quote = None;

    for ch in input.chars() {
        match (ch, in_string, current_quote) {
            // Handle string start
            ('"' | '\'', false, None) => {
                in_string = true;
                current_quote = Some(ch);
                current_value.push(ch);
            }
            // Handle string end
            ('"' | '\'', true, Some(q)) if ch == q && !escape => {
                current_value.push(ch);
                in_string = false;
                current_quote = None;
            }
            // Handle escaped quotes within strings
            ('"' | '\'', true, Some(q)) if escape => {
                current_value.push(ch);
                escape = false;
            }
            // Handle escape character
            ('\\', true, _) => {
                escape = true;
                current_value.push(ch);
            }
            // Handle commas outside of strings
            (',', false, None) => {
                // Process the completed value
                values.push(parse_single_value(&current_value.trim())?);
                current_value.clear();
            }
            // Accumulate characters
            _ => {
                current_value.push(ch);
                escape = false;
            }
        }
    }

    // Add the last value if not empty
    if !current_value.is_empty() {
        values.push(parse_single_value(&current_value.trim())?);
    }

    Ok(values)
}

/// Parse a single SQL value into SqlValue
fn parse_single_value(value: &str) -> Result<SqlValue, String> {
    // Remove surrounding quotes if present
    let unquoted = value.trim_matches(|c| c == '\'' || c == '"');

    // Try parsing as different types
    if unquoted.eq_ignore_ascii_case("NULL") {
        return Ok(SqlValue::Null);
    }

    // Try parsing as boolean
    if let Ok(bool_val) = unquoted.parse::<bool>() {
        return Ok(SqlValue::Boolean(bool_val));
    }

    // Try parsing as Number
    if let Ok(int_val) = unquoted.parse::<i64>() {
        return Ok(SqlValue::Number(int_val.to_string(), true));
    }

    // Default to string
    Ok(SqlValue::SingleQuotedString(unquoted.to_string()))
}