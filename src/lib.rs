extern crate csv;
extern crate rusqlite;

use rusqlite::Connection;
use rusqlite::types::ToSql;

use std::path::Path;
use std::fs;
use std::io;

fn load_table(path: &str, table_name: &str, conn: &Connection) {
    let mut rdr = csv::Reader::from_path(path).unwrap();
    let mut create = format!("CREATE TABLE {} (", table_name);
    let mut insert = format!("INSERT INTO {} (", table_name);
    let mut values = format!("VALUES (");
    let mut index = 0;
    for field in rdr.headers().unwrap() {
        if index > 0 {
            create.push_str(", ");
            insert.push_str(", ");
            values.push_str(", ");
        }
        index += 1;
        create.push_str(&format!("{} TEXT", field));
        insert.push_str(&format!("{}", field));
        values.push_str(&format!("?{}", index));
    }
    create.push_str(");");
    insert.push_str(") ");
    values.push_str(");");
    insert.push_str(&values.as_str());

    println!("{}", create);
    conn.execute(&create.as_str(), &[]).unwrap();
    println!("{}", insert);
    let mut stmt = conn.prepare(&insert.as_str()).unwrap();
    for record in rdr.records() {
        let record = record.unwrap();
        let args = "***ARG HELP ME I NEED TO COERCE record.iter() into a &[&ToSql]!!!***";
        stmt.execute(&args).unwrap();
    }
    let select = format!("SELECT * FROM {}", table_name);
    println!("{}", select);
    let mut rows = stmt.query(&[]).unwrap();
    while let Some(row) = rows.next() {
        let row = row.unwrap();
        let mut fields: Vec<String> = Vec::new();
        for i in 0..row.column_count() {
            fields.push(row.get(i));
        }
        println!("{:?}", fields);
    }

}

pub fn load_tables(dir: &str) -> io::Result<()> {
    let conn = rusqlite::Connection::open_in_memory().unwrap();
    let path = Path::new(dir);
    println!("path: {:?}", path);
    for entry in fs::read_dir(path)? {
        let dir = entry?;
        if !dir.path().is_dir() {
            let path = dir.path();
            let table_name =  path.file_stem().unwrap().to_str().unwrap();
            load_table(&dir.path().to_str().unwrap(), &table_name, &conn);
        }
    }
    Ok(())
}

