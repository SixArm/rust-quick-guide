# Cow type for clone-on-write

<https://doc.rust-lang.org/std/borrow/enum.Cow.html>

A Rust `Cow` type is a clone-on-write smart pointer. When a function receives a `Cow` type as an argument, the function can modify the data without actually modifying the original data structure. Instead, the `Cow` type makes a clone of the data when it is modified, and any other references to the original data continue to point to the original data.

The `Cow` type is implemented as an enum with two variants: `Borrowed` and `Owned`, which express "either a reference, or an owned type". You choose which variant you want depending on your goal.

```rust
use std::borrow::Cow;

fn main() {
    let a = ['a', 'b', 'c'];
    let mut b = Cow::Borrowed(&a);

    // The `b` Cow enum is borrowed, and points to `a`.
    match b {
        Cow::Borrowed(_) => println!("Borrowed"),
        Cow::Owned(_) => println!("Owned"),
    }

    // Convert `b` to mutable i.e. clone it, then change it.
    b.to_mut()[0] = 'x';

    // Now the `b` Cow enum is Owned i.e. has its own data.
    match b {
        Cow::Borrowed(_) => println!("Borrowed"),
        Cow::Owned(_) => println!("Owned"),
    }
}
```

A typical use case for `Cow` is optimization by not doing copies. For example, you write a function that returns a String, but there are cases when you already have a &'static str containing the data; you can return `Cow::Borrowed` so you don't need to allocate and copy a new String.
