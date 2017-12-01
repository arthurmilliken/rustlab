extern crate csv;

use std::path::Path;
use std::fs;
use std::io;

fn load_table(path: &str, table_name: &str) {
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
    println!("{}", insert);
    let mut count = 0;
    for _ in rdr.records() {
        // let record = record.unwrap();
        count += 1;
        if count % 10_000 == 0 {
            println!("  {}", count);
        }
    }
    let select = format!("SELECT count(*) FROM {}", table_name);
    println!("{}", select);
}

pub fn load_tables(dir: &str) -> io::Result<()> {
    let path = Path::new(dir);
    println!("path: {:?}", path);
    for entry in fs::read_dir(path)? {
        let dir = entry?;
        if !dir.path().is_dir() {
            let path = dir.path();
            let table_name =  path.file_stem().unwrap().to_str().unwrap();
            load_table(&dir.path().to_str().unwrap(), &table_name);
        }
    }
    Ok(())
}

