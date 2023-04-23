# epoll crate - example

[Runnable project](/projects/crates/epoll/tcp_listener)

Example to create a TCP listener and register it with epoll:

```rust
use std::{net::TcpListener, os::unix::io::AsRawFd};
use epoll::Epoll;
use epoll::{ControlOptions, Event, Events};

fn main() -> std::io::Result<()> {
    // Create an epoll instance and a listener fd
    let epoll = Epoll::new()?;
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    let listener_fd = listener.as_raw_fd();

    // Register the listener with epoll
    let mut event = Event::new(
        ControlOptions::EPOLLIN,
        listener_fd as u64
    );
    epoll.ctl_add(&mut event)?;

    let mut events = Events::with_capacity(128);
    loop {
        // Wait for events to occur
        let num_events = epoll.wait(&mut events, -1)?;
        for i in 0..num_events {
            let event = events.get(i).unwrap();
            if event.data() == listener_fd as u64 {
                // Accept the connection
                let (stream, _) = listener.accept()?;
                println!("Accepted new connection");
            }
        }
    }
}
```

The code creates an epoll instance, and a TCP listener, and registers it with epoll. Then the code uses an event loop, which waits for events to occur and processes them. The code accepts incoming connections and prints a message.
