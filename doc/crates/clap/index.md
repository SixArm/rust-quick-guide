# CLAP crate for commands

<https://crates.io/crates/clap>

The Rust CLAP crate is for command line argument parsing. CLAP provides a flexible and intuitive way to define command-line interfaces (CLIs) for Rust programs, with support for a wide range of features and options.

Add a dependency in your file `Cargo.toml` file, along with features you want:

```rust
[dependencies]
clap = { version = "4", features = ["derive"] }
```

Defining your CLI options by using CLAP derive `Parser`:

```rust
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "None")]
struct Args {
   #[arg(short, long)]
   name: String,
   #[arg(short, long)]
   age: i32,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}
```

Example run:

```sh
cargo run -- --name Alice --age 22
```

Example output:

```sh
Args { name: "Alice", age: 22 }
```
