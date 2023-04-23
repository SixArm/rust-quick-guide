# Parallelism with rayon

Rust has built-in support for parallelism, which is the ability to execute multiple tasks simultaneously on multiple processors or cores.

Rust's support for parallelism is especially easy to use by adding the Rust `rayon` crate, which provides a high-level API for parallel programming. The `rayon` crate allows developers to easily parallelize data processing tasks, such as iterating over large collections, by abstracting away the low-level details of thread creation and synchronization.

Here is an example code snippet that demonstrates Rust parallelism using rayon:

```rust
use rayon::prelude::*;

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let sum = numbers.par_iter().sum::<i32>();
    println!("Sum is {}", sum);
}
```

In this example, the `par_iter()` method creates a parallel iterator over a vector of numbers. The `sum()` method is then called on the iterator to calculate the sum of all the numbers in the vector.

`rayon` automatically divides the work among multiple threads, using as many threads as there are processors or cores available on the system. The code is executed in parallel, with each thread processing a subset of the data.

The `par_iter()` method can be used with many other methods of the standard library, such as `map()`, `filter()`, and `reduce()`, to parallelize various data processing tasks.
