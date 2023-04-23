# Tokio crate for async/concurrency

<https://crates.io/crates/tokio>

The Rust Tokio crate is a widely used library for building asynchronous and concurrent applications. It provides a runtime for executing asynchronous tasks and a set of libraries for building networking and other I/O-heavy applications.

Edit file `Cargo.toml`:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

Example to connect to a mini-redis server:

```rust
use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Open a connection to the mini-redis address.
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Set the key "hello" with value "world"
    client.set("hello", "world".into()).await?;

    // Get key "hello"
    let result = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);

    Ok(())
}
```
