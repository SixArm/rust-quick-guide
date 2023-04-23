use sqlx::{SqlitePool, Row};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let db_url = "sqlite::memory:";
    let pool = SqlitePool::connect(db_url).await?;
    sqlx::query("\
        CREATE TABLE IF NOT EXISTS users (\
        id INTEGER PRIMARY KEY NOT NULL,\
        name VARCHAR NOT NULL );\
    ").execute(&pool).await?;
    sqlx::query("\
        INSERT INTO users values (1, 'Alice'), (2, 'Bob');\
    ").execute(&pool).await?;
    let rows = sqlx::query("\
        SELECT * from users;"
    ).fetch_all(&pool).await?;
    for row in rows {
        println!("id {}, name {:?}", 
            row.get::<i32, &str>("id"),
            row.get::<String, &str>("name")
        );
    }
    Ok(())
}
