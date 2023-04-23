# cargo-release crate for publishing

<https://crates.io/crates/cargo-release>

The Rust cargo-release crate provides a set of tools for releasing Rust crates to repositories such as crates.io. It automates many of the steps involved in releasing a new version of a crate, making it easier and more efficient to manage the release process.

To use the `cargo-release` crate in your Rust project, you'll need to add it as a dependency in your `Cargo.toml` file. Once you've done that, you can configure the crate by creating a `.cargo` directory in your project root, then adding a `config.toml` file with the following contents:

```toml
[package]
version = "0.1.0"

[dependencies]
cargo-release = { version = "0.15", features = ["procmacro"] }

[release]
# ... configure release options here ...
```

Overall, the `cargo-release` crate provides a powerful and flexible set of tools for managing the release process for Rust crates. It can help to streamline the release process, reduce the risk of errors and inconsistencies, and ensure that your crates are published to repositories like crates.io in a consistent and reliable manner.


Features: Ensures you are in a good state for release, such as with your git branch, remote, and tree. Supports workspaces using cargo's native flags, like --workspace, --exclude and --package.  Handles cargo publish, tagging, and pushing.
