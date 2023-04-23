# Sealed traits

In Rust, a sealed trait is a trait that can only be implemented within a particular module, and not outside it. This means that once a trait is marked as "sealed", any other code outside the module where the trait was defined cannot implement it.

Sealed traits are useful when you want to limit the set of types that can implement a particular trait to a specific set of types. This can be helpful when designing APIs or libraries where you want to restrict the use of certain traits to specific contexts or modules.

To define a sealed trait in Rust, you must declare the trait as `pub` and include a private `mod` statement with the same name as the trait. This private module should contain all the implementations of the trait.

Here's an example:

```rust
pub trait Sealed {}

mod private {
    use super::Sealed;

    impl Sealed for i32 {}
    impl Sealed for String {}
}

pub fn foo<T: Sealed>(val: T) {
    // do something with val
}
```

In this example, the Sealed trait is defined as pub and the implementations are placed in a private module called private. The foo function is generic over T where T must implement the Sealed trait.

Because the private module is private, no other code outside the module can implement the Sealed trait, ensuring that only the types explicitly listed within the module can be used with the trait.
