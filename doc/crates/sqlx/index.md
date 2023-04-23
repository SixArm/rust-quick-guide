# sqlx crate for SQL databases

<https://crates.io/crates/sqlx>


The Rust sqlx crate provides a type-safe, asynchronous, and composable SQL interface for working with databases. It is designed to make working with databases in Rust easier and more productive, while still being fast and efficient.

sqlx supports a wide range of databases, including PostgreSQL, MySQL, SQLite, and Microsoft SQL Server. It uses Rust's type system to provide a safe and ergonomic way to write SQL queries, while still allowing for raw SQL queries if needed.

One of the key features is support for async/await syntax, which allows for non-blocking database queries and operations. This makes it easy to write efficient, high-performance database code in Rust that can handle large numbers of concurrent requests.

In addition to its core functionality, sqlx provides a number of other useful features, such as support for migrations, prepared statements, transactions, automatic type conversions, and more. It also has excellent documentation and a friendly and helpful community, which makes it easy to get started and solve problems as they arise.


## Dependencies

When you add sqlx to `Cargo.toml`, you must choose feature providers for your specific database, and for Transport Layer Security (TLS). Also, sqlx versions have different options, so be sure to check the documentation.

```toml
[dependencies]
sqlx = {
    version = "0.6",
    features = [ "sqlite", "runtime-tokio-rustls" ]
}
tokio = { version = "1", features = ["full"] }
```
