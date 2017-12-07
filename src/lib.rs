extern crate csv;
extern crate sqlite;

use std::fs;

fn load_table(path: &str, table_name: &str, conn: &sqlite::Connection) {
  let mut rdr = csv::Reader::from_path(path).unwrap();
  let mut create = format!("CREATE TABLE [{}] (", table_name);
  let mut insert = format!("INSERT INTO [{}] (", table_name);
  let mut values = format!("VALUES (");
  let mut index = 0;
  for field in rdr.headers().unwrap() {
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

  // Create table
  println!("{}", create);
  conn.execute(create).unwrap();

  // Inset records
  println!("{}", insert);
  let mut stmt = conn.prepare(insert).unwrap();
  let mut count = 0;
  for record in rdr.records() {
    count += 1;
    if count % 10_000 == 0 { println!("  {}", count) }
    if count > 1000 { break }

    let record = record.unwrap();
    stmt.reset().unwrap();
    for (column, field) in record.iter().enumerate() {
      stmt.bind(column + 1, field).unwrap();
    }
    stmt.next().unwrap();
  }
  let select = format!("SELECT count(*) as num FROM {};", table_name);
  println!("{}", select);
  conn.iterate(select, |record| {
    println!("  {:?}", record);
    true
  }).unwrap();
}

pub fn load_tables(path: &str, db: &str) {
  println!("\nload_tables():'{}':'{}'\n", path, db);
  if let (Ok(conn), Ok(read_dir)) = (
    sqlite::open(db), fs::read_dir(path)
  ) {
    for entry in read_dir {
      match entry {
        Ok(dir) => {
          let path = dir.path();
          if !path.is_dir() {
            if let (Some(table), Some(path)) = (
              path.file_stem().and_then(|s| s.to_str()),
              path.to_str()
            ) {
              load_table(&path, &table, &conn);
            }
            else {
              println!("error extracting table from path: {:?}", path);
            }
          }
        },
        Err(e) => println!("{:?}", e),
      }
    }
    // } else {
    //   println!("could not read dir: {}", path);
    // }

  } else {
    println!("could not open db: {}", db);
  }
}

