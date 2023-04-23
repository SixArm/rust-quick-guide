# CLAP command builder - example

See code directory `/projects/crates/clap`

The CLAP command builder pattern is one way set up CLAP:

```rust
use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("My Program")
    .version("1.0.0")
    .author("Alice Adams")
    .about("This is my program")
    .arg(
        Arg::new("name")
        .help("Set the name to use")
        .short('n')
        .long("name")
        .action(ArgAction::Set)
    )
    .after_help("Longer explanation")
    .get_matches();

    // Process the command line arguments
    if let Some(x) = matches.get_one::<String>("name") {
        println!("Name is {}", x);
    }
}
```

The command builder defines introduction program information, then the `name` argument, then any conclusion program information. The example uses the `get_matches()` method to parse command-line arguments into a `matches` struct, then prints the `name` argument.

Example run:

```sh
cargo run -- --name Alice
```

Example output:

```sh
Name is Alice
```
