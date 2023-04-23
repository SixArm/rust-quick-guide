use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open(":memory:")?;
    conn.execute(
        "CREATE TABLE person (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            age   INTEGER NOT NULL
        )",
        [],
    )?;
    conn.execute(
        "INSERT INTO person (id, name, age) VALUES (?1, ?2, ?3)",
        ["1", "Alice", "30"],
    )?;
    let name: String = conn.query_row(
        "SELECT name FROM person WHERE id=1",
        [],
        |row| row.get(0),
    )?;
    println!("{}", name);
    Ok(())
}

