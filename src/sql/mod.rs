extern crate csv;
extern crate sqlite;

mod writer;

use std::fs;
use std::result;
use std::io;

#[derive(Debug)]
enum Error {
  CsvError(csv::Error),
  SqlError(sqlite::Error),
  IoError(io::Error),
  // Message(String),
}

impl From<csv::Error> for Error {
  fn from(e: csv::Error) -> Self {
    Error::CsvError(e)
  }
}

impl From<sqlite::Error> for Error {
  fn from(e: sqlite::Error) -> Self {
    Error::SqlError(e)
  }
}

impl From<io::Error> for Error {
  fn from(e: io::Error) -> Self {
    Error::IoError(e)
  }
}

type Result<T> = result::Result<T, Error>;

fn load_table(path: &str, table_name: &str, conn: &sqlite::Connection) {
  if let Err(e) = || -> Result<()> {
    let mut rdr = csv::Reader::from_path(path)?;
    writer::Writer::new(table_name, &mut rdr, conn)?.write()?;
    Ok(())
  }() {
    println!("error writing table at {:?}, {:?}", path, e);
  }
}

pub fn load_tables(path: &str, db: &str) {
  println!("\nload_tables():'{}':'{}'\n", path, db);
  if let Err(e) = || -> Result<()> {
    let conn = sqlite::open(db)?;
    for entry in fs::read_dir(path)? {
      let path = entry?.path();
      if !path.is_dir() {
        if let (Some(table_name), Some(file)) = (
          path.file_stem().and_then(|s| s.to_str()),
          path.to_str(),
        ) {
          load_table(&file, &table_name, &conn);
        }
        else {
          println!("error extracting table from path: {:?}", path);
        }
      } else {
        println!("path {:?} is a directory.", path);
      }
    }
    Ok(())
  }() {
    println!("{:?}", e);
  }
}

pub fn run() {
  println!("run:");
  if let Err(e) = || -> sqlite::Result<()> {
    let conn = sqlite::open(":memory:")?;
    conn.execute("CREATE TABLE Person (id INT, name TEXT);")?;

    let mut stmt = conn.prepare("INSERT INTO Person(id, name) VALUES (?1, ?2);")?;

    stmt.bind(1, 1)?;
    stmt.bind(2, "Arthur")?;
    stmt.next()?;

    stmt.reset()?;
    stmt.bind(1, 2)?;
    stmt.bind(2, "Wayne")?;
    stmt.next()?;

    stmt.reset()?;
    stmt.bind(1, 3)?;
    stmt.bind(2, "Milliken")?;
    stmt.next()?;

    stmt.reset()?;
    stmt.bind(1, 0)?;
    stmt.bind(2, ())?;
    stmt.next()?;

    conn.iterate("SELECT * FROM Person;", |record| {
      let mut row = String::new();
      let mut first = true;
      for &(field, value) in record.iter() {
        if !first { row.push_str(", "); }
        let value = match value {
          Some(v) => format!("'{}'", v),
          None => String::from("null"),
        };
        row.push_str(&format!("{}: {}", field, value));
        first = false;
      }
      println!("  {}", row);
      true
    })?;
    Ok(())
  }() {
    println!("  {:?}", e);
  }
}
