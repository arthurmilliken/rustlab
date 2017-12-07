extern crate csv;
extern crate sqlite;

mod writer;

use std::fs;

fn load_table(path: &str, table_name: &str, conn: &sqlite::Connection) {
  match csv::Reader::from_path(path) {
    Ok(mut rdr) => {
      match writer::Writer::new(table_name, &mut rdr, conn) {
        Ok(mut writer) => {
          if let Err(e) = writer.write() {
            println!("error writing table {}: {:?}", table_name, e);
          }
        },
        Err(e) => println!("error reading headers at {:?}: {:?}", path, e),
      }
    },
    Err(e) => println!("error opening csv reader at {:?}: {:?}", path, e),
  }
}

pub fn load_tables(path: &str, db: &str) {
  println!("\nload_tables():'{}':'{}'\n", path, db);
  if let (Ok(conn), Ok(read_dir)) = (sqlite::open(db), fs::read_dir(path)) {
    for item in read_dir {
      match item {
        Ok(entry) => {
          let path = entry.path();
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
          } else {
            println!("path {:?} is a directory.", path);
          }
        },
        Err(e) => println!("{:?}", e),
      }
    }
  } else {
    println!("could not open db: {}", db);
  }
}

