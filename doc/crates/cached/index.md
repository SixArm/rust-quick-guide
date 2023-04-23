# cached crate for memoization

The Rust `cached` crate provides implementations of several caching structures as well as macros for defining memoized functions.

Example to cache a function's input and output:

```rust
use cached::proc_macro::cached;

/// Defines a function named `fib` that uses cached.
/// By default, the cache name will be the function's
/// name in all caps; this cache name is "FIBONACCI".
#[cached]
fn fiboncacci(n: u64) -> u64 {
    if n == 0 || n == 1 { return n }
    fiboncacci(n-1) + fiboncacci(n-2)
}
```

Example to cache a function call for a time period:

```rust
use cached::proc_macro::once;

/// Only cache the initial function call.
/// Function will be re-executed after the cache
/// expires (according to `time` seconds).
/// When no (or expired) cache, concurrent calls
/// will synchronize (`sync_writes`) so the function
/// is only executed once.
#[once(time=10, option = true, sync_writes = true)]
fn keyed(a: String) -> Option<usize> {
    if a == "a" {
        Some(a.len())
    } else {
        None
    }
}
```
