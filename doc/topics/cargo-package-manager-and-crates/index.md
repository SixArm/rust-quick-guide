# Cargo package manager and crates

In Rust, Cargo is the package manager and build tool that creates and manages projects and their dependencies. Cargo provides ways to easily build, test, document, and publish code.

Cargo uses a file called `Cargo.toml` to manage the configuration and dependencies of a Rust project. The `Cargo.toml` file specifies the name of the package, version information, and the dependencies of the project. Cargo also provides a command-line interface that allows developers to manage their Rust projects and dependencies easily.

A cargo package is called a "crate". A crate can be a binary or a library. A binary crate is an executable program. A library crate is code that can be used by other programs.

Cargo provides a standardized directory structure for Rust projects. By convention, the main source code of a project is placed in a directory called src, and the project configuration and dependencies are specified in a file called `Cargo.toml`. Cargo uses the `Cargo.lock` file to keep track of exact dependency versions used in the project.

Cargo also provides a number of commands to manage a Rust project. Some of the commonly used commands include:

* `cargo new`: Create a new Rust project.

* `cargo build`: Build the project and its dependencies.

* `cargo run`: Build and run the project.

* `cargo test`: Run the project tests.

* `cargo doc`: Generates documentation for the project.

* `cargo publish`: Publishes a crate to the official registry.
