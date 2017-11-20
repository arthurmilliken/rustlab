extern crate csv;

use std::path::Path;
use std::fs;
use std::io;

pub fn read_headers(path: &str) {
    let mut rdr = csv::Reader::from_path(path).unwrap();
    {
        let headers = rdr.headers().unwrap();
        println!("    {:?}", headers);
    }
    // for result in rdr.records() {
    //     let row = result.unwrap();
    //     println!("row: {:?}", row);
    // }    
}

pub fn read_dir(dir: &str) -> io::Result<()> {
    let path = Path::new(dir);
    println!("path: {:?}", path);
    for entry in fs::read_dir(path)? {
        let dir = entry?;
        if !dir.path().is_dir() {
            let path = dir.path();
            println!("{:?}", path.file_stem().unwrap());
            read_headers(dir.path().to_str().unwrap());
        }
    }
    Ok(())
}