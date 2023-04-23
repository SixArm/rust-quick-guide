# Test-driven development - example

For this test-driven development example, imagine you want to write a function `foo` that always returns `true`.

First write a test:

```rust
#[cfg(test)]           // Annotation: the mod is for cargo test
mod tests {            // Define a module named "tests"
    use super::*;      // Use code from the outer module

    #[test]            // Annotation: this function is a test
    fn foo_test() {    // Define a function as usual
      assert!(foo());  // The assert! test macro must be true
    }

}
```

Verify the test failure:

```sh
cargo test
```

Write the simplest code possible to pass the test:

```rust
pub fn foo() -> bool { // Define a function
    true // Always return true
}
```

Verify the test success:

```sh
cargo test
```

Rust has built-in test assertion macros such as `assert`, `assert_eq`, `assert_ne`. In practice, these are fine for simple TDD, but may be too basic for real-world TDD. In our real-world projects, we use the Assertables crate that provides many more assertions such as `assert_starts_with`, `assert_contains`, and `assert_is_match`.

<https://crates.io/crates/assertables>
