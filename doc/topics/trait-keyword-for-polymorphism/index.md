# trait keyword for polymorphism

In Rust, a trait is a language construct that defines a set of methods that can be implemented by a type. Traits enable polymorphism, generic programming, and code reuse without sacrificing performance or safety.

Example trait that defines one method:

```rust
trait MyTrait {
    fn my_method(&self);
}
```

Example struct that implements the method:

```rust
struct MyStruct;

impl MyTrait for MyStruct {
    fn my_method(&self) {
        println!("Hello");
    }
}
```

Example function that takes the trait and calls the method:

```rust
fn foo<T: MyTrait>(item: T) {
    item.my_method();
}
```

To run it:

```rust
fn main() {
    let s = MyStruct{};
    foo(s)
}
```

Some of the common Rust traits are `Debug` and `Display` for formmating output, `Copy` and `Clone` for duplicating values, `From` and `Into` for converting values, and `Send` and `Sync` for multi-thread communication.
