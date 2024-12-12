mod btree;
mod table;
mod parser;

use std::fmt::Debug;
use std::io::{self, Write};
use crate::parser::parse_and_execute;

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
            Ok(_) => print!(""),
            Err(e) => eprintln!("Error: {}", e)
        }
    }
}


