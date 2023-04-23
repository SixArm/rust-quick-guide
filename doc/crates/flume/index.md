# Flume crate for channels

<https://crates.io/crates/flume>

The Rust Flume crate provides multi-producer, multi-consumer channels, including unbounded, bounded, and rendezvous queues.

Flume is fast, flexible, and a drop-in replacement for std::sync::mpsc, with additional features like MPMC support, send timeouts/deadlines, and an ergonomic select-like interface.

Example to spawn and sum:

```rust
use std::thread;

fn main() {
    println!("Spawn");
    let (tx, rx) = flume::unbounded();
    thread::spawn(move || {
        (0..10).for_each(|i| {
            tx.send(i).unwrap();
        })
    });
    let received: u32 = rx.iter().sum();
    assert_eq!((0..10).sum::<u32>(), received);
}
```
