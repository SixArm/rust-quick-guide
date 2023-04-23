# What is a Rust "Fibonacci" program?

A "Fibonacci" program is a job interview challenge: given an index number `n`, print its Fibonacci sequence term.

The Fibonacci sequence is a mathematical sequence in which each number is the sum of the two preceding ones: 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, etc. For example, index number `0` is the sequence term `0`, and index number `12` is the  sequence term `144`. 

The example below defines a function named `fibonacci`. The function takes an input which is the Fibonacci sequence index number, and returns the Fibonacci sequence term number. For example, input index `0` returns term `0`, and input index  `12` returns term `144`.

Example:

```rust
pub fn fibonacci(n: usize) -> usize {
    match n {
        0 => 0,
        1 => 1,
        2.. => fibonacci(n - 1) + fibonacci(n - 2),
        _ => panic!("{}", n)
    }
}

fn main() {
    println!("{}", fibonacci(12));
}
```

Output:

```text
144
```

The "Fibonacci" program is frequently seen in programming for benchmarking, because the calculations and the recursions grow very quickly, and also because there are a variety of ways to optimize the program to run faster, to work without recursions, and to use less memory. For more about this, see the Rust Guideposts page "Memoize a function with cached".

