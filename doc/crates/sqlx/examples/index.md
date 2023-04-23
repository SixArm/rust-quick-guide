# sqlx crate - example

[Runnable project](/projects/crates/sqlx/create_table_insert_into_select)

Example to create a table, insert data, and select data:

```rust
use sqlx::{SqlitePool, Row};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Create an in-memory database
    let db_url = "sqlite::memory:";

    // Create a database connection pool
    let pool = SqlitePool::connect(db_url).await?;

    // Create a table
    sqlx::query("\
        CREATE TABLE IF NOT EXISTS users (\
        id INTEGER PRIMARY KEY NOT NULL,\
        name VARCHAR NOT NULL );\
    ").execute(&pool).await?;

    // Insert data
    sqlx::query("\
        INSERT INTO users values (1, 'Alice'), (2, 'Bob');\
    ").execute(&pool).await?;

    // Select data
    let rows = sqlx::query("\
        SELECT * from users;"
    ).fetch_all(&pool).await?;

    // Print data
    for row in rows {
        println!("id {}, name {:?}",
            row.get::<i32, &str>("id"),
            row.get::<String, &str>("name")
        );
    }
    Ok(())
}
```
