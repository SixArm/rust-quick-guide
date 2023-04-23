# Access a database with rusqlite

Rust example to connect to a SQLite database and execute SQL queries, by using the `rusqlite` crate:

```rust
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
```

This example creates a `Connection` to a SQLite database in memory, creates a table named "people", inserts data into it, selects data from it, and prints it out. The `rusqlite` crate provides many more SQL features, such as transactions and prepared statements.
