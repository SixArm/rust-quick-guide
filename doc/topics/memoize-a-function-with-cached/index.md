# Memoize a function with cached

The phrase "memoize a function" means "create a cache of the function's inputs and outputs". This is called memoizaiton, and can significantly speed up some functions, especially ones that do large calculations, or ones that do recursive calculations.

An easy way to memoize a function is to use the Rust `cached` crate. 

Example:

```rust
use cached::proc_macro::cached;

#[cached]
fn fibonacci(n: usize) -> usize {
    if n == 0 || n == 1 { return n }
    fibonacci(n-1) + fibonacci(n-2)
}
```

The example defines a function named `fibonacci`. The Fibonacci sequence is a mathematical sequence in which each number is the sum of the two preceding ones: 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, etc. For example, index number `0` is the sequence term `0`, and index number `12` is the  sequence term `144`. 

The `fibonacci` function takes an input which is a Fibonacci sequence index number, and returns the Fibonacci sequence term number. For example, input index `0` returns term `0`, and input index  `12` returns term `144`.

By default, the function's memoizaiton cache will be the function's name in all caps i.e. `FIBONACCI`.
