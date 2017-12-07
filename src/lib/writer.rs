extern crate csv;
extern crate sqlite;

use std::fs::File;

use self::csv::{ Reader, StringRecord };
use self::sqlite::{ Connection, State, Statement };

pub struct Writer<'a> {
  create: String,
  insert: String,
  select_count: String,
  conn: &'a Connection,
  rdr: &'a mut Reader<File>,
}

impl<'a> Writer<'a> {
  pub fn new(table_name: &str,
         rdr: &'a mut Reader<File>,
         conn: &'a Connection) -> csv::Result<Writer<'a>> {
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

    Ok(Writer { create, insert, select_count, conn, rdr })
  }

  pub fn write(&mut self) -> sqlite::Result<()> {
    // Create table
    println!("{}", self.create);
    self.conn.execute(&self.create)?;

    // Insert records
    println!("{}", self.insert);
    let mut stmt = self.conn.prepare(&self.insert)?;
    let mut count = 0;
    for record in self.rdr.records() {
      count += 1;
      if count % 10_000 == 0 { println!("  {}", count) }
      if count > 1000 { break } // DELETE ME!

      match record {
        Ok(row) => if let Err(e) = Writer::write_row(&row, &mut stmt) {
          println!("  *** error writing row: {:?}", e);
        },
        Err(e) => println!("  *** error reading row: {:?}", e),
      }
    }
    println!("{}", self.select_count);
    let mut stmt = self.conn.prepare(&self.select_count)?;
    while let State::Row = stmt.next()? {
      println!("  count: {}", stmt.read::<i64>(0)?);
    }
    Ok(())
  }

  fn write_row(row: &StringRecord, stmt: &mut Statement) -> sqlite::Result<()> {
    stmt.reset()?;
    for (column, field) in row.iter().enumerate() {
      stmt.bind(column + 1, field)?;
    }
    stmt.next()?;
    Ok(())
  }
}