# console, dialoguer, indicatif for CLIs

<https://crates.io/crates/console>

<https://crates.io/crates/dialoguer>

<https://crates.io/crates/indicatif>

[project](/projects/crates/console/hello_world)
  
The `console` crate provides access to terminal features so you can build nicer looking command line interfaces. 

The `dialoguer` crate helps you build small user inputs for the command line, such as prompts, inputs, selections, history, and more.

The `indicatif` crate helps you build command line interfaces that report progress to users. It helps format anything that indicates progress.

Example:

```rust
use console::Term;
use dialoguer::Input;
use indicatif::ProgressBar;

fn main() -> std::io::Result<()> {
    // console example
    let term = Term::stdout();
    term.write_line("Hello, World!")?;

    // dialoguer example
    let input = Input::<String>::new().interact_text()?;
    term.write_line(&input)?;

    // indicatif example
    let bar = ProgressBar::new(10);
    for _ in 0..10 { bar.inc(1); }
    bar.finish();

    Ok(())
}
```
