# Display trait for formatting

The Rust `Display` trait is a built-in trait that allows developers to format a value as a string for display purposes. It provides a human-readable representation of a type.

When a type implements the `Display` trait, it can be formatted as a string using the `format!` macro or the `println!` macro with the `{}` format specifier.

To implement the `Display` trait for a custom type, we define a `fmt` method on the type that takes a formatter object. The formatter object implements the `fmt::Write` trait, which provides methods for writing to a string buffer.

Example:

```rust
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // The `write!` macro writes fields into the formatter
        write!(f, "x is {} and y is {}", self.x, self.y)
    }
}

fn main() {
    let p = Point { x: 1, y: 2 };
    println!("Point {}", p); // "Point x is 1 and y is 2"
}
```
Here, we define a `fmt` method for the `Display` trait on our `Point` struct. This method takes a formatter object, and writes the `x` field and `y` field of the struct into a user-friendly string.
