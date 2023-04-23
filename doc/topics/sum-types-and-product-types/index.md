# Sum types and product types

In Rust, sum types and product types are two fundamental concepts in algebraic data types, which are used to define custom data structures.


## Sum type

A sum type is a type that can have one of several possible values. In Rust, sum types are defined using the `enum` keyword. An enum can have one or more variants, each of which can contain different types of data.

Example sum type:

```rust
enum Color {
    Red,
    Green,
    Blue,
}
```

In this example, `Color` is a sum type that combines three variants into a single type.


## Product type

A product type is a type that combines two or more types into a single type. In Rust, product types are defined using the tuple syntax, which looks like (T1, T2, ..., Tn). The resulting type can be thought of as a record that contains values of each of the individual types.

Example product type:

```rust
struct Point {
    x: i32,
    y: i32,
    z: i32,
}
```

In this example, `Point` is a product type that combines three i32 values into a single type.

