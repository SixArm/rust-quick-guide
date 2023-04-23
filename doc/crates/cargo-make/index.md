# cargo-make crate for task runners

<https://crates.io/crates/cargo-make>

The Rust cargo-make crate is a tool that extends the functionality of the Cargo package manager by providing a way to define complex build processes in a simple, declarative way.

Here are some of the key features of the cargo-make crate:

* Declarative build scripts: With cargo-make, you define your build process in a Toml configuration file, which makes it easy to understand and modify the build process.

* Cross-platform support: cargo-make runs on Linux, macOS, and Windows, making it easy to maintain consistent build processes across different platforms.

* Task management: You can define a set of tasks, each of which can be executed individually or as part of a larger build process.

* Dependency management: cargo-make ensures that tasks are executed in the correct order based on their dependencies, which helps avoid build errors and improve build performance.

* Pre and Post Hooks: cargo-make supports pre- and post-hooks, to perform actions before and after the build process, such as cleaning artifacts, setting environment variables, etc.

* Plugins: cargo-make supports plugins to extend functionality, such as adding new tasks or modifying the build process.

Install:

```sh
cargo install cargo-make
```

After installation, you can define your build process in a Toml configuration file named `Makefile.toml`.
