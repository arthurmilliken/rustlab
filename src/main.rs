mod lib;
mod sql;

use std::env;

const BASE_DIR: &str = "./KJV-database/CSV/";

fn main() {
    match env::args().nth(1) {
        Some(arg) => {
            match arg.as_str() {
                "hello" => println!("cmd: hello"),
                "load" => {
                    match lib::load_tables(BASE_DIR) {
                        Ok(_) => println!("done!"),
                        Err(e) => println!("ERROR: {:?}", e),
                    }
                },
                "sql" => sql::run(),
                _ => println!("unknown cmd: {}", arg),
            }
        }
        None => println!("usage: <prg> <cmd>"),
    }
}

// #[derive(Debug)]
