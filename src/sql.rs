extern crate rusqlite;
use self::rusqlite::*;
// use std::path::Path;

// #[derive(Debug)]
pub fn run() {
    // let path = Path::new("./test.db");
    // let conn = Connection::open(path).unwrap();
    let conn = Connection::open_in_memory().unwrap();
    conn.execute("CREATE TABLE Person (id TEXT, name TEXT);", &[]).unwrap();
    conn.execute("INSERT INTO Person(id, name) VALUES (?1, ?2);", &[&"1", &"Arthur"]).unwrap();
    conn.execute("INSERT INTO Person(id, name) VALUES (?1, ?2);", &[&"2", &"Wayne"]).unwrap();
    conn.execute("INSERT INTO Person(id, name) VALUES (?1, ?2);", &[&"3", &"Milliken"]).unwrap();

    let mut stmt = conn.prepare("SELECT * FROM Person;").unwrap();

    let mut rows = stmt.query(&[]).unwrap();
    while let Some(row) = rows.next() {
        let row = row.unwrap();
        let mut fields: Vec<String> = Vec::new();
        for i in 0..row.column_count() {
            fields.push(row.get(i));
        }
        println!("found: {:?}", fields);
    }

    println!("done.");
}