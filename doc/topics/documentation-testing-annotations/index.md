# Documentation testing annotations

Documentation comment code blocks can use annotations with attributes that help `rustdoc` do the right thing when testing your code. Here are the annotations.

This test must panic:

```rust
/// ```should_panic
/// assert!(false);
/// ```
```

This test must compile, but is not run:

```rust
/// ```no_run
/// assert!(true);
/// ```
```

This test must fail to compile:

```rust
///  ```compile_fail
/// snafu
/// ```
```

This test is only for Rust 2018 edition:

```rust
/// ```edition2018
/// assert!(true);
/// ```
```

This code block is ignored, and not a test:

```rust
///  ```ignore
/// This is something else besides a test.
/// ```
```

This code block is text, and not a test:

```rust
///  ```text
/// Hello, world!
/// ```
```
