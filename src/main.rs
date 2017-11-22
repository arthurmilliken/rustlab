mod lib;

const BASE_DIR: &str = "./KJV-database/CSV/";

fn main() {
    match lib::load_tables(BASE_DIR) {
        Ok(_) => println!("done!"),
        Err(e) => println!("ERROR: {:?}", e),
    }
}

// #[derive(Debug)]
