# List directories recursively with walkdir

Rust example code to list directories recursively with the walkdir crate.

Example:

```rust
use walkdir::WalkDir;

fn main() {
    let path = "/usr";
    for entry in WalkDir::new(path)
    .max_depth(3)
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

The example function does these steps:

1. Start at the system directory path "/usr".

2. Create a WalkDir object. Limit the walk to maximum depth of 3 directories. Use an iterator. Filter-map the results to be entries that are ok.

3. For each entry, see if it's a directory or file, and print its path, such as "Directory: /usr/bin" or "File: /usr/bin/true", etc.
