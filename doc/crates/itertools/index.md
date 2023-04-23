
# itertools crate for iterator extras

<https://crates.io/crates/itertools>

The Rust itertools crate is a third-party library that provides a powerful set of tools for working with iterators in Rust. It offers a wide range of functions and macros for manipulating and combining iterators, making it easier and more efficient to work with collections of data in Rust.

The itertools crate provides:

* iteration functions that can be used to manipulate and transform iterators

* combinator functions that can be used to generate new iterators from existing iterators

* macros that can be used to simplify the code required to work with iterators

Example:

```rust
use itertools::{Itertools, join};

fn main() {
    // Demo data
    let numbers = vec![1, 2, 3];
    let letters = vec!['a', 'b', 'c'];

    // Use the "join" macro to flatten lists into one string
    let joined = join(letters, ", ");
    println!("{:?}", joined);

    // Use the combinator functions to mix iterators
    for (n, l) in numbers.iter()
        .cartesian_product(letters.iter()) {
        println!("{}{}", n, l);
    }
}
```
