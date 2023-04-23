# What is a Rust "FizzBuzz" program?

A "FizzBuzz" program is a job interview challenge: print the numbers 1 to 100, except replace any multiple of 3 with "Fizz", any multiple of 5 with "Buzz", and any multiple of both 3 and 5 with "FizzBuzz".

One way to write FizzBuzz:

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

The example uses a `for` loop, `if`...`else` control flow statements, `%` modulo operator, `&&` logical operator, and `println!` macros to print output.

FizzBuzz output should start with these lines:

```text
1
2
Fizz
4
Buzz
Fizz
7
8
Fizz
Buzz
11
Fizz
13
14
FizzBuzz
```
