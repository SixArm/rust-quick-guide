# Closures for iterators

Rust closures can be used to create iterators, which are a powerful tool for working with collections of data. 

For example, the `map` method on a Vec can be used to apply a closure to each element of the vector:

```rust
let numbers = vec![1, 2, 3, 4,6, 7, 8];
let squares = numbers.iter().map(|x| x * x);
for square in squares {
    println!("Square number is {}", square);
}
```

In this example, we define a vector of numbers, then use the `iter` method to create an iterator over the vector's elements. We then use the `map` method to apply a closure that squares each element of the vector. Finally, we loop over the resulting iterator and print each square number.

For example, the `filter` method on a Vec can be used to select each element that matches a condition:

```rust
// Use the `filter` method
let numbers = vec![1, 2, 3, 4,6, 7, 8];
let evens = numbers.iter().filter(|x| *x % 2 == 0);
for even in evens {
    println!("Even number is {}", even);
}
```

In this example, we define a vector of numbers, then use the `iter` method to create an iterator over the vector's elements. We then use the `filter` method to apply a closure that squares each element of the vector. Finally, we loop over the resulting iterator and print each even number.
