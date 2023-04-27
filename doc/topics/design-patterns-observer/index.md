# Design patterns: observer

The "observer" design pattern enables one object to notify others of its state changes. This can be implemented using Rust's channels or event emitters.

Example:

```rust
use std::sync::mpsc::channel;
use std::thread;

fn main() {
    let (tx, rx) = channel();

    thread::spawn(move || {
        tx.send("Hello, World!").unwrap();
    });

    let message = rx.recv().unwrap();
    println!("{}", message);
}
```
