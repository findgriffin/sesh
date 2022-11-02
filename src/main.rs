use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Record {
    id: i32,
    input: String,
    output: Option<String>,
}
  
  

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;
    conn.execute("CREATE TABLE log (id INTEGER PRIMARY KEY, output TEXT, input TEXT NOT NULL)", ())?;
    let one = Record {
        id: 0,
        input: "ls".to_string(),
        output: Some("hello.txt".to_string()),
    };
    conn.execute("INSERT INTO log (input, output) VALUES (?1, ?2)",
		 (&one.input, &one.output))?;
    let mut stmt = conn.prepare("SELECT id, input, output FROM log")?;
    let record_iter = stmt.query_map([], |row| {
        Ok(Record {id: row.get(0)?, input: row.get(1)?, output: row.get(2)?})
	})?;
  
    for record in record_iter {
        println!("Found record {:?}", record.unwrap());
    }
    Ok(())
}
