# Assertables crate for assert macro tests

<https://crates.io/crates/assertables>

The Rust Assertables crate is a library of assert macros for testing, validation, and verification. If an assert macro succeeds, then it completes normally, otherwise it prints a diagnostic error message.

Edit your file `Cargo.toml`:

```toml
[dependencies]
assertables = "7"
```

Example of how to use the Assertables crate:

```rust
#[cfg(test)]
mod test_assert_x_result {
    use assertables;

    #[test]
    fn example1() {
        let x = 1;
        let y = 2;
        assert_lt!(x, y);
    }

    #[test]
    fn example2() {
        let string1 = "Hello World";
        let string2 = "He";
        assert_starts_with!(string1, string2);
    }
}
```

In the example, the macro `assert_lt!` tests that `x` is less than `y`, and the macro `assert_starts_with!` tests that `string1` starts with `string2`.

The Assertable crate provides a range of macros for compile-time testing, as well as debug macros for non-optimized runtime debugging, and runtime macros for optimized runtime validation and verification.
