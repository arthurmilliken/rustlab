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
      "run" => sql::run(),
      "load" => sql::load_tables(BASE_DIR, ":memory:"),
      unknown => println!("unknown cmd: {}", unknown),
    }
  } else {
    println!("usage: {} <cmd>", prog);
  }
}
// #[derive(Debug)]
