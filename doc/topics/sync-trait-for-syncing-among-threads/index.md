# Sync trait for syncing among threads

The Rust `Sync` trait indicates that a type is safe to be shared between multiple threads. If a type implements the `Sync` trait, it can be safely accessed from multiple threads without causing any data races or undefined behavior. For example, the `Arc` type implements the `Sync` trait, so it can be safely shared between multiple threads.

Here's an example of how the Sync trait can be used:

```rust
use std::sync::Arc;

struct MyStruct {
    x: i32,
}

impl MyStruct {
    fn my_function(&self) {
        println!("x is {}", self.x);
    }
}

fn main() {
    let s = MyStruct { x: 1 };
    let shared = Arc::new(s);
    std::thread::spawn({
        let shared = shared.clone();
        move || { shared.my_function(); }
    }).join().unwrap();
}
```

In this example, we create a shared instance of `MyStruct` using the `Arc` type, which automatically implements the `Sync` trait. We can then safely access the shared instance from multiple threads, without worrying about synchronization issues.
