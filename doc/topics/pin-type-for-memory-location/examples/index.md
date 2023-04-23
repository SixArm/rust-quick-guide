# Pin type - example

Here's an example of how to use the Rust Pin type:

```rust
use std::pin::Pin;

struct Data {
    value: i32,
}

impl Data {
    fn new(value: i32) -> Self {
        Self { value }
    }
}

fn main() {
    let data = Data::new(1);
    let pinned_data = Pin::new(&data);

    // Invalid move of `data`:
    // let moved_data = data;

    // Invalid move of `pinned_data`:
    // let moved_pinned_data = pinned_data;

    // We can access the value of `data` through `pinned_data`
    println!("{}", pinned_data.value);
}
```

In this example, we define a Data struct that holds a single integer value. We then create a new instance of this struct, and use the `Pin::new` function to create a Pin wrapper around a reference to this instance.

Once `pinned_data` is created, trying to move data results in a compile-time error. Similarly, attempting to move `pinned_data` results in a compile-time error, because it is a wrapper around a pinned reference.

We can still access the value of data through `pinned_data`, as shown by the `assert_eq!` statement. The reference remains valid, even if the data structure itself is moved.
