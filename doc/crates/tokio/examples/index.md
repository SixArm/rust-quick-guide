# Tokio crate - example HTTP server

[Runnable project](/projects/crates/tokio/http_server)

You can use Tokio to build network applications, such as an HTTP server:

```rust
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];
            let n = socket.read(&mut buf).await.unwrap();
            let request = String::from_utf8_lossy(&buf[..n]);
            println!("Received request:\n{}", request);

            let response = "HTTP/1.1 200 OK\r\n\r\nHello!";
            socket.write_all(response.as_bytes())
            .await.unwrap();
        });
    }
}
```

This defines a main function that binds to port 8080 and listens for incoming TCP connections.

When a connection is accepted, a new task is spawned to handle the request asynchronously.

The task reads the incoming data from the socket, prints it to the console, and sends a response back to the client.
