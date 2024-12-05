mod btree;
mod table;

use std::io::{self, Write};
use crate::btree::btreenode::Row;
use crate::table::table::Table;

fn main() {
    let mut table = Table::new();
    loop {
        print!("db > ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let trimmed = input.trim();

        if trimmed.starts_with("insert") {
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            // to fix!!!
            if let (Ok(id), data) = (parts[1].parse(), parts.get(2).unwrap().to_string()) {
                let row = Row { id, data};
                table.insert_row(row);
            }
        } else if trimmed == "select" {
            table.print_rows();
        } else if trimmed == ".exit" {
            break;
        } else {
            println!("Unknown command.");
        }
    }
}
