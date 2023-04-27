# gtk4 crate - example

[Runnable project](/projects/crates/gtk4/hello_world)

Example "Hello, World!" using GTK from the docs:

```rust
use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();

    app.connect_activate(|app| {
        // We create the main window.
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(200)
            .title("Hello, World!")
            .build();

        // Show the window.
        window.present();
    });

    app.run()
}
```

Notes:

* The gtk4 crate is usually renamed to gtk. You can find an example in the features section for how to do this globally in your Cargo.toml.

* GTK needs to be initialized before use by calling the function `init`. When you create an `Application` struct, this will call `init` for you.
