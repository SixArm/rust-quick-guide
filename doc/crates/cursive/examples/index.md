# Cursive crate - example

[Runnable project](/projects/crates/cursive/hello_world)

Run a simple interactive terminal user interface program, by using the `cursive` crate.

```rust
use cursive::{Cursive, CursiveExt};
use cursive::views::{Dialog, TextView};

fn main() {
    let mut siv = Cursive::default();

    siv.add_layer(
        Dialog::around(TextView::new("Hello, world!"))
            .title("Cursive Example")
            .button("Quit", |s| s.quit()),
    );

    siv.run();
    println!("Ok")
}
```

This code creates a `Cursive` object, adds a `TextView` containing the message "Hello, world!" to a `Dialog`, and then displays the dialog with a "Quit" button that will close the application when clicked.
