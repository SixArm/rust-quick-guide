# Textwrap crate - example

[Runnable project](/projects/crates/textwrap/fill_wrap)

Example of textwrap fill and wrap:

```rust
use textwrap::{fill, wrap};

fn main() {
    let s = "Rust is a great programming language for us";
    println!("{}", fill(s, 22));
    println!("{:?}", wrap(s, 22));
}
```

Example output:

```text
Rust is a great
programming language
for us
["Rust is a great", "programming language", "for us"]
```

In this example, we import the `fill` and `wrap` functions from the textwrap crate. 

We use `fill` to fill one string with lines that are each 22 characters maximum per line. 

We use `wrap` to create a vector of strings that are each 22 characters maximum per line.
