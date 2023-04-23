# Panic and how to handle it with a hook

In Rust, a `panic` occurs when a program encounters a situation where it cannot continue to run safely. This can happen for a variety of reasons, such as a failed assertion, an out-of-bounds array access, or an attempt to unwrap a `None` value. When a `panic` occurs, Rust will unwind the stack and search for a `catch_unwind` block that can handle the panic. If no such block is found, the program will terminate with an error message.

By default, Rust will print an error message and terminate the program when a panic occurs. However, it is possible to customize this behavior by adding a `panic` hook. This allows you to define your own `panic` handler that can log the error, send an alert, or perform other actions before terminating the program.

You define a `panic` hook by calling the `std::panic::set_hook` function. Pass a closure that takes a `PanicInfo` struct as an argument; the struct contains useful debugging information.

Example of a panic hook that logs the message then terminates:

```rust
use std::panic;

fn main() {
    panic::set_hook(Box::new(|panic_info| {
        let message = panic_info
            .payload()
            .downcast_ref::<String>()
            .unwrap_or(&"Unknown error".to_string());
        eprintln!("Panic occurred: {}", message);
    }));
    panic!("Yikes!"); // Deliberately trigger a panic
}
```

This sets a `panic` hook that logs the `panic` message to the standard error stream using the `eprintln` macro. When the program encounters a `panic!` macro, it will trigger the panic hook and log the error message before terminating the program.
