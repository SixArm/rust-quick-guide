# cargo-make crate - example

Here's an example `cargo-make` configuration file `Makefile.toml`:

```toml
[tasks.build]
command = "cargo build --release"

[tasks.test]
command = "cargo test"

[tasks.lint]
command = "cargo clippy"

[tasks.default]
dependencies = ["build", "test", "lint"]
```

In this example, we've defined three tasks: `build`, `test`, and `lint`. Each task has a command that specifies what action to perform when the task is executed. The `default` task depends on the `build`, `test`, and `lint` tasks, and is executed when no task is specified.

You can then run your build process using the following command:

```sh
cargo make
```

This will execute the default task and all its dependencies in the correct order.

If you want to execute a specific task, you can use the following command:

```sh
cargo make <task-name>
```
