# macro_rules! for declarative macros

<https://doc.rust-lang.org/book/ch19-06-macros.html>

The Rust `macro_rules!` macro is a powerful code generation tool that allows the developer to create custom syntax or keywords that expand into Rust code at compile time. With this macro, you can define custom syntax rules, patterns, and templates that can be used to generate code automatically.

The `macro_rules!` macro works by defining a set of rules that match the input code, similar to a regular expression. These rules are then used to generate Rust code based on the input, which can be used to reduce the amount of repetitive or boilerplate code required for a given codebase.

Syntax:

```rust
macro_rules! my_macro_name {
  // Define patterns and templates here that match the input code
}
```

Here's an example of a simple Rust macro that generates a for loop with a range of numbers:

```rust
#[macro_export]
macro_rules! number_loop {
    ($start:expr, $end:expr) => {
        for i in $start..$end {
            println!("{}", i);
        }
    }
}
```

With this macro, you can now generate a for loop by simply invoking the number_loop! macro with the desired start value and end value:

```rust
number_loop!(0, 10);
```

This will output the numbers from 0 to 9.
