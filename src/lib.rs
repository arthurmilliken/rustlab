extern crate csv;
extern crate sqlite;

use std::path::Path;
use std::fs;
// use std::io;

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
    for _ in rdr.records() {
        let record = record.unwrap();

        count += 1;
        if count % 10_000 == 0 { println!("  {}", count) }
        if count > 1000 { break }
    }
    let select = format!("SELECT count(*) FROM {};", table_name);
    println!("{}", select);
}

pub fn load_tables(dir: &str) {
    let conn = sqlite::open(":memory:").unwrap();
    let path = Path::new(dir);
    println!("path: {:?}", path);
    for entry in fs::read_dir(path).unwrap() {
        let dir = entry.unwrap();
        if !dir.path().is_dir() {
            let path = dir.path();
            let table_name =  path.file_stem().unwrap().to_str().unwrap();
            load_table(&dir.path().to_str().unwrap(), &table_name, &conn);
        }
    }
}

