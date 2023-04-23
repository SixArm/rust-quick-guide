# From and Into traits for conversions

The Rust `From` trait and `Into` trait are used to convert between types.

The `From` trait provides a `from` method that takes an argument of a different type and returns an instance of the implementing type. This allows for easy conversion between different types, especially when converting from a type that is not owned by the implementing type.

The `Into` trait provides an `into` method that takes no arguments and returns an instance of a different type. This allows for easy conversion between different types, especially when converting from a type that is owned by the implementing type.

Example:

```rust
struct MyStruct(i32);

// Convert from i32
impl From<i32> for MyStruct {
    fn from(val: i32) -> Self {
        MyStruct(val)
    }
}

// Convert into i32
impl Into<i32> for MyStruct {
    fn into(self) -> i32 {
        self.0
    }
}

fn main() {
    let my_struct = MyStruct::from(42);
    let i: i32 = my_struct.into();
}
```

This example defines a simple `MyStruct` struct. We implement the `From` trait `from` method. We implement the `Into` trait `into` method.
