# mod keyword for nested hierarchies

The Rust `mod` keyword can provide nested hierarchies, meaning that a modules can contain other modules:

```rust
pub mod outer {
    pub mod inner {
        pub fn hello() {
            println!("Hello");
        }
    }
}

fn main() {
    outer::inner::hello()
}
```

You can optionally add a `use` statement such as:

```rust
use outer::inner::hello;
fn main() {
    hello()
}
```

Module hierarchies can help test-driven development, because you can create an outer module `tests`, with an inner module for each function, to improve readability and encapsultation:

```rust
#[cfg(test)]
mod tests {
    mod my_function_1 {
        #[test]
        fn test_something() {
            assert!(/* ... */);
        }
    }
    /* ... */
}
```
