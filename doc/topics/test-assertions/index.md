# Test assertions

The Rust testing framework provides macros for test assertions, such as:

* `assert!(condition)`: assert `condition` is true.

* `assert_eq!(a, b)`: assert `a` is equal to `b`.

* `assert_ne!(a, b)`: assert `a` is not equal to `b`.

Example:

```rust
let x = 1;
let y = 2;
assert!(x < y);
```

Example with an optional message:

```rust
let x = 1;
let y = 2;
assert!(x < y, "We want x to be less than y");
```


## Assertables crate

The Assertables crate provides more assert macros, such as:

* `assert_starts_with!(x, y)`: Does `x` start with `y`?

* `assert_contains!(array, element)`: Does `array` contains `element`?

* `assert_is_match!(regex, string)`: Does `regex` match `string`?
  
Example:

```rust
use assertables;
let a = "hello world";
let b = "hello";
assert_starts_with!(&a, &b);
```
