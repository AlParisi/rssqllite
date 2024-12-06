mod btree;
mod table;

use std::io::{self, Write};
use crate::btree::btreenode::Row;
use crate::table::table::Table;
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use sqlparser::ast::{Statement, Expr};

fn main() {

    loop {
        print!("db > ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let query = input.trim();

        if query.eq_ignore_ascii_case("exit") || query.eq_ignore_ascii_case("quit") {
            break;
        }

        match parse_and_execute(query) {
            Ok(_) => println!(" "),
            Err(e) => eprintln!("Error: {}", e)
        }
    }
}


fn parse_and_execute(query: &str) -> Result<(), String> {
    let mut table = Table::new();
    let dialect = GenericDialect {};
    let statements = Parser::parse_sql(&dialect, query)
        .map_err(|e| format!("Failed to parse SQL: {}", e))?;

    for statement in statements {
        match statement {
            Statement::CreateTable (create) => {
                println!("Creating table '{}'", create.name);
                for column in create.columns {
                    println!(" - Column: {} ({:?})", column.name, column.data_type);
                }
            }
            Statement::Insert( insert) => {
                //need to looping on Identifier to fix row values
                println!("Insert sources: {:?}", insert.source);
                if let (id, data) = (1, insert.source) {
                    let row = Row { id, data};
                    table.insert_row(row);
                }
            }
            _ => {
                println!("Unsupported statement: {:?}", statement);
            }
        }
        //still to fix this!
        if query.eq_ignore_ascii_case("select"){
            table.print_rows();
        }
    }

    Ok(())
}