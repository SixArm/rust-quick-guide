# Hyper crate - example

[Runnable project](/projects/crates/hyper/hello_world)

Example "Hello, World!" server:

```rust
use std::convert::Infallible;

async fn handle(
    _: hyper::Request<Body>
) -> Result<hyper::Response<hyper::Body>, Infallible> {
    Ok(hyper::Response::new("Hello, World!".into()))
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let make_svc = hyper::service::make_service_fn(|_conn| async {
        Ok::<_, Infallible>(hyper::service::service_fn(handle))
    });

    let server = hyper::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
```
