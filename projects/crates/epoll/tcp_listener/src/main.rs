#[cfg(target_os = "linux")]
fn main() -> std::io::Result<()> {
    use std::{net::TcpListener, os::unix::io::AsRawFd};
    use epoll::{Epoll, ControlOptions, Event, Events};

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

#[cfg(not(target_os = "linux"))]
fn main() {
    println!("Skip")
}
