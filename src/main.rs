mod sql;

use std::env;

const BASE_DIR: &str = "./KJV-database/CSV/";

fn main() {
  let mut args = env::args();
  let prog = match args.nth(0) {
    Some(p) => p,
    None => String::from("rustlab"),
  };

  if let Some(cmd) = args.nth(0) {
    match cmd.as_str() {
      "hello" => println!("Hello, World!"),
      "sql" => sql::run(),
      "load" => {
        if let Some(db) = args.nth(0) {
          sql::load_tables(BASE_DIR, db.as_str());
        } else {
          sql::load_tables(BASE_DIR, ":memory:");
        }
      },
      unknown => println!("unknown cmd: {}", unknown),
    }
  } else {
    println!("usage: {} <cmd>", prog);
  }
}
// #[derive(Debug)]
