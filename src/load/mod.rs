extern crate csv;
extern crate rusqlite;

use self::rusqlite::types::{ ToSql };
use std::error::Error;
use std::fs::File;
use std::result;

const PATH: &str = "./KJV-database/CSV/Books.csv";

type Result<T> = result::Result<T, Box<Error>>;

#[derive(Debug)]
struct Schema {
  create: String,
  insert: String,
  select: String,
  select_count: String,
}

impl Schema {
  pub fn load(table_name: &str, rdr: &mut csv::Reader<File>) -> Result<Schema> {
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

    let select = format!("SELECT * FROM [{}];", table_name);
    let select_count = format!("SELECT count(*) FROM [{}];", table_name);
    Ok(Schema { create, insert, select, select_count })
  }
}

fn load_table(name: &str, path: &str) -> Result<Schema> {
  let mut rdr = csv::Reader::from_path(path)?;
  let schema = Schema::load(name, &mut rdr)?;

  let conn = rusqlite::Connection::open_in_memory()?;
  println!("{}", schema.create);
  conn.execute(&schema.create, &[])?;
  for record in rdr.records() {
    let row = record?;
    println!("{}", schema.insert);
    let params: Vec<&str> = row.iter().collect();
    println!("            {:?}", params);
    conn.execute(&schema.insert, params.as_slice())?; // THIS LINE FAILS
  }
  println!("{}", schema.select_count);
  let count: i32 = conn.query_row(&schema.select_count, &[], |row| { row.get(0) })?;
  println!("count: {:?}", count);
  Ok(schema)
}

pub fn run() {
  match load_table("Books", PATH) {
    Ok(_) => println!("done!"),
    Err(err) => println!("error: {}", err),
  }
}