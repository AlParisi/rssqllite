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
            if parts.len() == 4 {
                if let (Ok(id), Some(username), Some(email)) = (parts[1].parse(), parts.get(2), parts.get(3)) {
                    let row = Row { id, username: username.to_string(), email: email.to_string() };
                    table.insert_row(row);
                } else {
                    println!("Invalid command format.");
                }
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
