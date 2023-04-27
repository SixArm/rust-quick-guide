# What is a Rust "Hello, World!" program?

In Rust, a simple "Hello, World!" program is:

```rust
fn main() {
    println!("Hello, World!");
}
```

This program contains a single function, `main()`, which is the entry point for the program. The function body is enclosed in curly braces `{}` and contains a single statement:

```rust
println!("Hello, World!");
```

This statement prints the text "Hello, World!" to the console using Rust's standard library macro `println!()`. The `println!()` macro is a convenient way to print formatted text to the console, and in this case, it simply prints the string literal "Hello, World!".

When you run this program, you should see the text "Hello, World!" printed to the console.

To create this program, the typical way is to use the Rust `cargo` package manager, which can create an example project:

```sh
cargo new hello
cd hello
```

Then edit the `src/main.rs` file, which is automatically created with the "hello world" code above. Change it as you wish.

To run the program:

```sh
cargo run
```

You should see the output:

```txt
Hello, World!
```
