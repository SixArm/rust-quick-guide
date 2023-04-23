# rand crate for random numbers

The Rust `rand` crate is a library that provides various random number generators (RNGs), algorithms, and utilities. It can generate random numbers of different types (such as integers or floating-point numbers), as well as random bytes and strings.

The crate offers several algorithms for generating random numbers:

* Xorshift: a fast, non-cryptographic algorithm that generates random numbers with a period of 2^128 - 1.

* ChaCha: a stream cipher that can be used to generate random numbers with a very long period, suitable for cryptographic applications.

* Hc128: another stream cipher that can be used for random number generation.

In addition to generating random numbers, the "rand" crate also provides utilities for shuffling arrays, generating random values from enums, and more.

## rand crate example

```rust
use rand::Rng;

fn main() {
    // Create a random number generator
    let mut rng = rand::thread_rng();
    let number = rng.gen_range(1..=100);
    println!("{}", number);
}
```

In this example, we use the `Rng` trait from the rand crate to generate a random number between 1 and 100. The `thread_rng()` function returns a new instance of the generator. The `gen_range()` function generates a random number in the specified range (inclusive of the lower bound and exclusive of the upper bound). We print the result to the console.