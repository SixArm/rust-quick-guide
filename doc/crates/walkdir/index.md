# walkdir crate for traversing directories

The Rust `walkdir` crate provides ways to iterate over directories and their contents. It is useful for traversing directories, such as for file managers, build systems, or search engines. It is built on top of the `std::fs` module.

Key features of the `walkdir` crate include: recursive directory iteration with configurable maximum depth; filtering options based on file attributes or name patterns; error handling and recovery mechanisms for I/O errors or permission issues; configurable follow-symlinks behavior; support for custom sorting and ordering of entries; optional support for cross-platform path handling and case sensitivity.

Example of how to use the walkdir crate:

```rust
use walkdir::WalkDir;

fn main() {
    for entry in WalkDir::new("/path/to/directory")
    .into_iter()
    .filter_map(|e| e.ok()) {
        if entry.file_type().is_dir() {
            println!("Directory: {}", entry.path().display());
        } else {
            println!("File: {}", entry.path().display());
        }
    }
}
```

The `WalkDir::new` function creates a new directory walker, and `into_iter` returns an iterator that can be filtered and mapped over. The `ok` method filters out any errors that may occur during iteration. Then the `file_type` method on the `entry` variable checks if the entry is a directory or a file. Finally, we print out the name of the entry using the `display` method.
