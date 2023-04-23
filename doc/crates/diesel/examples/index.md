# Diesel crate - example

[Runnable project](/projects/crates/diesel/hello_world_with_sqlite)

The Diesel crate is sophisticated because its typical use involves SQL migrations, database connections, automatic conversions from records to structs, and much more. The Diesel tutorial is excellent and well worth reading. This page has elided excerpts, to give you a taste.

Diesel uses schema macros, typically in a file `schema.rs`:

```rust
diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
    }
}
```

Diesel uses model structs, typically in a file `models.rs`:

```rust
#[derive(Debug, Queryable, Identifiable, AsChangeset)]
pub struct Post {
    pub id: i32,
    pub title: String,
}
```

Diesel uses database connections such as:

```rust
let mut connection = 
    SqliteConnection.establish("db.sqlite")
    .expect("Error connecting to database");
```

Diesel uses Domain-Specific Language (DSL) such as:

```rust
use self::schema::posts::dsl::*;
let results = posts
    .load::<Post>(connection)
    .expect("Error loading posts");
for post in results {
    println!("{}", post.title);
}
```
