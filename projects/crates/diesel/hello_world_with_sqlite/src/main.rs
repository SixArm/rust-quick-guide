use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use self::models::*;

// Diesel migrations for persistent databases are typically via Diesel CLI.
// Diesel migrations for non-persistent databases can embed migrations.
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub mod models;
pub mod schema;

fn main() {
    use self::schema::posts::dsl::*;
    let conn = 
        &mut SqliteConnection::establish("database.sqlite").unwrap();
    conn.run_pending_migrations(MIGRATIONS).unwrap();

    // Delete existing posts
    diesel::delete(posts::table)
        .execute(conn)
        .unwrap();

    // Insert a new post
    use crate::schema::posts;
    let new_post = NewPost { id: 1, title: "Hello, World!" };
    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(conn)
        .expect("Error: insert new post");

    // Select all posts
    let results = posts
        .load::<Post>(conn)
        .expect("Error: load posts");
    for post in results {
        println!("{}", post.title);
    }
}
