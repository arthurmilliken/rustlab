mod lib;
mod sql;

use std::env;

const BASE_DIR: &str = "./KJV-database/CSV/";

fn main() {
    match env::args().nth(1) {
        Some(arg) => {
            match arg.as_str() {
                "hello" => println!("cmd: hello"),
                "load" => lib::load_tables(BASE_DIR),
                "sql" => sql::run(),
                "iterate" => lib::iterate(),
                _ => println!("unknown cmd: {}", arg),
            }
        }
        None => println!("usage: <prg> <cmd>"),
    }
}

// #[derive(Debug)]
