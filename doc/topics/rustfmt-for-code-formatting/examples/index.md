# Rustfmt - examples

## Rustfmt as a standalone tool

You can use Rustfmt directly from the command line:

```sh
rustfmt <filename.rs>
```
This command will format the Rust code in the specified file and print the formatted output to the terminal. If you want to save the formatted output to a file, you can use the -w option followed by the filename, like this:

```sh
rustfmt -w <filename.rs>
```

## Rusfmt within a code editor

You can use Rustfmt within a code editor such as vim, emacs, Helix, and VSCode. To do this, you install a Rustfmt extension for your editor, then configure it to format your code on save or on demand.

For example, in VSCode, you can install the "Rustfmt" extension and configure it to format your code on save by adding the following line to your `settings.json` file:

```json
"editor.formatOnSave": true
```

## Rustfmt via a build script

You can use Rustfmt as a step of your build process, before compiling it. One way to do this is to create a build script by adding the following line to your `Cargo.toml` file:

```toml
[package]
build = "rustfmt <filename.rs>"
```
