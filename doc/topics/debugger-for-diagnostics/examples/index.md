# Debugger for diagnostics - example

To use a debugger for diagnostics, you can use Rust's built-in debugger, which called GDB (GNU Debugger).

Activate debug symbols. GDB requires debug symbols to provide human-readable information about the code, such as function and variable names, file names, and line numbers. To enable debug symbols in a release, you can add the following lines to their Cargo.toml file:

```toml
[profile.release]
debug = true
```

Build the program with debugging information. You can then build their Rust code with the debug option.

```sh
cargo build --bin my_program --release
```

Start GDB by typing `gdb my_program` in the terminal (where my_program is the name of the executable file).

Load the executable by using the command `file my_program` to load your program into GDB.

Set breakpoints. You can place breakpoints in your Rust code using the `break` command. For instance, to stop the program when reaching line 10 of the file `main.rs`, you can type `break main.rs:10`.

Run the program. You can execute the program by typing `run`. The execution will pause at the first breakpoint, if there is one.

Inspect variables. You can use the `print` command to check the value of a variable at a specific point in the code. For example, `print x`.

Use backtrace. You can use the `bt` command to print a backtrace of the stack, which shows the sequence of function calls that led to the current state of the program.

Continue execution of the program by using the command `continue`.
