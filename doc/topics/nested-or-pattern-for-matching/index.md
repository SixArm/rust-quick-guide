# Nested-or-pattern for matching

[Source](https://www.reddit.com/r/rust/comments/12sbjyj/power_of_the_operator_in_pattern_matching/)

The nested-or-pattern for matching combines `|` expressions.

Example `if` statement without nested-or-pattern:


```rust
if let Some(2) | Some(3) | Some(5) | Some(7) = value {…}
```

And with nested-or-pattern:

```rust
if let Some(2 | 3 | 5 | 7) = value …
```

The nested-or-pattern can be useful in many kinds of statements.

Example `match` statement:

```rust
match value {
    Some(n @ (2 | 3 | 5 | 7)) => println!("{n} is a prime"),
    …
```

Example `let` statement:

```rust
let (Ok(i) | Err(i)) = [1, 2, 3].binary_search(&2);
```

Example function definition:

```rust
fn f((Ok(i) | Err(i)): Result<i32, i32>) {}
```
