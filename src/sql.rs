extern crate sqlite;

// #[derive(Debug)]
pub fn run() {
  match try() {
    Ok(()) => println!("run(): done."),
    Err(e) => {
      let code = match e.code {
        Some(n) => { n },
        None => { 0 },
      };
      let msg = match e.message {
        Some(m) => { m },
        None => { String::new() },
      };
      println!("sqlite::Error({}): {}", code, msg);
    }
  }
}

fn try() -> sqlite::Result<()> {
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
}