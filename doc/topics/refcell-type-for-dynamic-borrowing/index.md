# RefCell type for dynamic borrowing

The Rust `RefCell` type is a container type that provides dynamic borrow checking at runtime, allowing for mutable or immutable borrows of its inner value based on certain rules. For example, there are cases where runtime borrow checking is necessary, such as when a value needs to be mutated within a shared reference.

* Mutable borrows: `RefCell` provides mutable borrows of an inner value through the use of its `borrow_mut` method. This method returns a mutable reference (`&mut`) to the inner value, which can be modified. However, `borrow_mut` will panic at runtime if there are any outstanding references (mutable or immutable) to the inner value.

* Immutable borrows: `RefCell` provides immutable borrows through its `borrow` method, which returns an immutable reference (`&`) to the inner value. Multiple immutable references can be outstanding at the same time, but attempting to call borrow_mut while there are outstanding immutable references will cause a panic.

Example:

```rust
use std::cell::RefCell;

fn main() {
    let x = RefCell::new(1);

    // Borrow a mutable reference to x's inner value
    let mut mutable_ref = x.borrow_mut();
    *mutable_ref = 2;

    // Borrow an immutable reference to x's inner value
    let immutable_ref = x.borrow();
    println!("The value of x is: {}", *immutable_ref);
}
```