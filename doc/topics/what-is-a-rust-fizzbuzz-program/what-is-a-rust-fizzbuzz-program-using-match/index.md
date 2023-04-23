# What is a Rust "FizzBuzz" program using match?

The "FizzBuzz" program can be implemented a variety of ways.

One way uses `if`...`else` statements, such as:

```rust
for i in 1..=100 {
    if i % 3 == 0 && i % 5 == 0 {
        println!("FizzBuzz");
    } else if i % 3 == 0 {
        println!("Fizz");
    } else if i % 5 == 0 {
        println!("Buzz");
    } else {
        println!("{}", i);
    }
}
```

Another way uses the `match` keyword and statement, which matches conditions, like this:

```rust
for i in 1..=100 {
    match i {
        i if i % 3 == 0 && i % 5 == 0 => println!("FizzBuzz"),
        i if i % 3 == 0 => println!("Fizz"),
        i if i % 5 == 0 => println!("Buzz"),
        i => println!("{}", i),
    }
}
```

Another way uses `match` keyword and statement with a tuple, which is a collection of values, like this:

```rust
for x in 1..=100 {
    match (x % 3 == 0, x % 5 == 0) {
        (true, true) => println!("FizzBuzz"),
        (true, _) => println!("Fizz"),
        (_, true) => println!("Buzz"),
        _ => println!("{}", x),
    }
}
```
