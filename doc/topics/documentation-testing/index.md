# Documentation testing

Rust doc tests are a form of Rust's testing framework that allows developers to include tests in the documentation of their code. This enables developers to write code examples and tests in the documentation itself, ensuring that the documentation remains up-to-date and accurate.

Example:

```rust
/// This is a document comment with a doc test.
///
/// This doc test must succeed.
/// 
/// ```
/// assert!(true);
/// ```
```

To run all the doc tests:

```sh
cargo test --doc
```

To also show warnings:

```sh
cargo test --doc -- --show-output
```

Rust doc tests have a variety of options to make them more powerful and more flexible.

* Annotations enable you to specify code blocks that should be ignored, or should panic, or should be compiled but not run. 

* Embedded comments enable you to write code that is hidden, so your documentation is shorter and more readable.
  
* Trailing returns enable you to skip lengthy error handling, and instead use `?` error returns.
