# gtk4 crate for GTK GUIs

<https://crates.io/crates/gtk4>

The Rust gtk4 crate is a set of bindings to the GTK4 graphical user interface toolkit. GTK4 is a widely used toolkit for creating cross-platform graphical user interfaces. GTK helps developers build modern desktop applications that are runnable on Linux, Windows, and macOS.

The crate provides a set of APIs for building GTK GUIs, including building widgets such as buttons, labels, text fields, menus, scrolling windows, styled look and feel, and more. The crate also offers some Rust-specific abstractions, such as closures and iterators, to make working with the GTK4 toolkit more ergonomic for Rust.

For a gentle introduction to Rust and GTK otegher,  we recommend the online book GUI development with Rust and GTK 4.

<https://gtk-rs.org/gtk4-rs/stable/latest/book/>

Caveats:

* You may need to install additional software. For example, on macOS you can install GTK4 via brew by running `brew install gtk4`.

* GTK is not thread-safe. Accordingly, none of the crate's structs implement the `Send` trait or `Sync` trait.

* The GTK bindings are well supported, although you'll often need to use the C documentation, according to Blessed.rs.

* By default this crate provides only GTK 4.0 APIs. You can access additional functionality by selecting one of the `v4_2`, `v4_4`, etc. features. Take care when choosing the version to target: some of your users might not have easy access to the latest ones.
