# Implicit lifetimes

Lifetimes can be implicit, meaning they do not need notations, or explicit, meaning they do need notations.

Implicit lifetimes are typical, because the Rust compiler can figure out lifetimes for many typical needs, which means the developer doesn't need to write the lifetimes.

Suppose a function has one input arg and an output reference:

```rust
fn trim_period(s: &String) -> &str {
    s.trim_matches('.')
}
```

The compiler can figure out lifetimes by using the function signature:

* The compiler sees the output string slice, and knows that the string slice memory must come from somewhere.
  
* The compiler sees the function has only one input arg, so knows that the memory must come from the input arg. 
  
* Therefore the compiler can automatically set the lifetimes: the input arg must live at least as long as the output string slice.

The compiler's implicit lifetimes are equivalent to these explicit lifetimes:

```rust
fn trim_period<'a>(s: &'a String) -> &'a str {
    s.trim_matches('.')
}
```
