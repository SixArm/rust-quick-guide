# struct keyword for custom data types

A Rust `struct` is a custom data type that groups related data and functions. A struct is defined using the `struct` keyword, followed by the name of the struct, and a block of curly braces that contains the fields of the struct.

Here is an example of a struct in Rust:

```rust
struct Rectangle {
    width: u32,
    height: u32,
}
```

Usage:

```rust
let r = Rectangle {
    width: 10,
    height: 20
};
```

Structs can also have functions associated with them, called methods. Methods are defined within the block of curly braces after the fields of the struct, and can be used to operate on the data within the struct.

Example methods:

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

This example uses `impl` to define an implementation block for the `Rectangle` struct, and defines a method named `area` that calculates the area of the rectangle. The `&self` parameter indicates that the method takes a reference to the struct as its first argument.
