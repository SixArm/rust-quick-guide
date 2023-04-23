# union keyword for multi-type memory

A Rust union is a user-defined type that is similar to a struct, but instead of each field having its own memory space, a union has a single memory space that can be interpreted as different types depending on the current value of the union.

To define a union, use the union keyword, then the name of the union, then the fields of the union. For example:

```rust
union MyUnion {
    i: i32,
    f: f32,
}
```

In this example, `MyUnion` has two fields: `i` and `f`. The union can hold one of these fields at a time, not both. When you access a field of a union, Rust ensures the field is the active field of the union.

To change the value, use the unsafe keyword and transmute function. For example:

```rust
unsafe {
    let mut my_union = MyUnion { i: 42 };
    my_union.f = std::mem::transmute(3.14f32);
}
```

In this example, we use `std::mem::transmute` to convert a `f32` value into a bit pattern that can be interpreted as an `i32`. We then assign this value to `my_union.f`. Because we haven't accessed `my_union.i` since it was set, Rust considers `my_union.f` to be the active field of the union. If try to access `my_union.i` now, it would be undefined behavior (UB).

Because unions represent different types in the same memory space, it's easy to accidentally create bugs. In general, only use unions when you need to work with low-level data structures or when you need to optimize memory usage.
