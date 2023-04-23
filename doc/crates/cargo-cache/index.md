# cargo-cache crate for caching builds

<https://crates.io/crates/cargo-cache>

The Rust cargo-cache crate provides a command-line interface (CLI) for managing the cache directory used by the Cargo package manager.

When you use Cargo to build a Rust project, it downloads and caches dependencies, build artifacts, and other files related to the build process in a directory called "cargo-cache". Over time, this directory can become quite large, taking up valuable disk space on your system.

The `cargo-cache` crate provides several commands that allow you to manage the cache directory. Some of the key features:

* Listing the contents of the cache directory

* Clearing the cache directory

* Showing the size of the cache directory

* Displaying information about individual cached packages

Example listing:

```
Cargo cache '~/.cargo':
Total:                               4.41 GB
  75 installed binaries:           481.75 MB
  Registry:                          3.92 GB
    Registry index:                503.26 MB
    2563 crate archives:           403.60 MB
    2563 crate source checkouts:     3.02 GB
  Git db:                            2.67 MB
    1 bare git repos:              905.51 KB
    1 git repo checkouts:            1.77 MB
```

Using `cargo-cache`, you can easily clear out old or unnecessary cached files, reclaiming valuable disk space on your system. You can also use the `cargo-cache` CLI to better understand the contents of the cache directory and diagnose any issues related to the build process.
