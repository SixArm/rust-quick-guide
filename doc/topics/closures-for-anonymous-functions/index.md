# Closures for anonymous functions

Closures are a type of anonymous function that can capture variables from their surrounding environment, and create self-contained units of behavior that can be passed around and reused.

Example of a closure in Rust:

```rust
let add = |a, b| a + b;
let result = add(3, 4);
```

In this example, we define a closure `add` that takes `a` and `b` and returns their sum. We call the closure with `3` and `4` and print the result.

Closures in Rust are defined using the `|` symbol to specify the arguments, followed by the body in braces `{}`. Rust's type inference system allows you to omit the types of the arguments, if they can be inferred.

Example of a closure that accesses a variable outside of it:

```rust
let x = 5;
let add_x = |y| x + y;
let result = add_x(3);
```

In this example, we define a closure `add_x` that takes an argument `y` and adds it to the variable `x` that is already defined outside of the closure. When we call the closure with argument `3`, it captures the value of `x` and returns `8`.

Example of a closure for collection iterator map function:

```rust
let numbers = vec![1, 2, 3, 4];
let squares = numbers.iter().map(|x| x * x);
```

In this example, we define a vector of numbers, then use the `iter` method to create an iterator over the vector's elements. We then use the `map` method to apply a closure that squares each element of the vector.
