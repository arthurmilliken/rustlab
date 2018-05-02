extern crate csv;

use std::error::Error;
use std::fs::File;
use std::result;

const PATH: &str = "./KJV-database/CSV/Books.csv";

type Result<T> = result::Result<T, Box<Error>>;

#[derive(Debug)]
struct Schema {
  create: String,
  insert: String,
  select_count: String,
}

fn create_schema(table_name: &str, rdr: &mut csv::Reader<File>) -> Result<Schema> {
  let mut create = format!("CREATE TABLE [{}] (", table_name);
  let mut insert = format!("INSERT INTO [{}] (", table_name);
  let mut values = format!("VALUES (");
  let mut index = 0;
  for field in rdr.headers()? {
    if index > 0 {
      create.push_str(", ");
      insert.push_str(", ");
      values.push_str(", ");
    }
    index += 1;
    create.push_str(&format!("[{}] TEXT", field));
    insert.push_str(&format!("[{}]", field));
    values.push_str(&format!("?{}", index));
  }
  create.push_str(");");
  insert.push_str(") ");
  values.push_str(");");
  insert.push_str(&values.as_str());

  let select_count = format!("SELECT count(*) FROM [{}];", table_name);
  Ok(Schema { create, insert, select_count })
}

fn load_table(name: &str, path: &str) -> Result<()> {
  let mut rdr = csv::Reader::from_path(path)?;
  let schema = create_schema(name, &mut rdr)?;
  // TODO: execute create
  println!("{}", schema.create);
  for record in rdr.records() {
    let row = record?;
    println!("{}", schema.insert);
    println!("            {:?}", row);
    // TODO: execute insert
  }
  println!("{}", schema.select_count);
  // TODO: execute count
  Ok(())
}

pub fn run() {
  match load_table("Books", PATH) {
    Ok(_) => println!("done!"),
    Err(err) => println!("error: {}", err),
  }
}