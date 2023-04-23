# Send trait for sending among threads

The Rust `Send` trait indicates that a type is safe to be sent across thread boundaries. This means that if a type implements the `Send` trait, it can be safely passed from one thread to another without causing any data races or undefined behavior. For example, the `String` type in Rust implements the `Send` trait, which means it can be safely shared across multiple threads.

To implement the `Send` trait for a custom type, all of its fields must also implement the `Send` trait. This is because if a type contains non-`Send` fields, it may be possible for data races to occur when the type is shared across threads. The `Send` trait is automatically implemented for most primitive types in Rust, as well as many standard library types like `Vec` and `String`.

Here's an example of a custom type that implements the `Send` trait:

```rust
struct Foo {
    x: i32,
    y: String,
}

unsafe impl Send for Foo {}

fn main() {
    let foo = Foo { x: 42, y: "Hello".to_string() };
    std::thread::spawn(move || {
        println!("x = {}, y = {}", foo.x, foo.y);
    }).join().unwrap();
}
```

In this example, the `Foo` struct contains an `i32` field and a `String` field. Both `i32` and `String` implement the `Send` trait, so we can implement `Send` for `Foo` using the `unsafe impl Send for Foo {}` syntax. We can safely send a `Foo` instance to a new thread using `std::thread::spawn`, and access its fields from within the thread.
