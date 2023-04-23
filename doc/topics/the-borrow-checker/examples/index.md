# The borrow checker - example

The borrow checker guarantees that an immutable borrow never changes data. This guarantee enables you to have multiple immutable borrows of the same object simultaneously.

```rust
let mut a = ['a', 'b', 'c'];

let b = &a; // Borrow data immutably
//b[0] = 'x'; // Changing data won't compile
println!("{:?}", b[0]); // Reading data is fine

let c = &mut a; // Borrow data mutably
c[0] = 'x'; // Changing data is fine.
println!("{:?}", c[0]); // Reading data is fine.
```

The borrow checker guarantees that mutable borrows of the same object never overlap. This guarantee protects you from accidentally doing conflicting mutations in mutable borrows.

```rust
let mut a = ['a', 'b', 'c'];

// Valid code because the mutable borrows are one at a time.
let b = &mut a;
b[0] = 'x';
let c = &mut a;
c[0] = 'y';

// Invalid code because the mutable borrows overlap.
//let b = &mut a;
//let c = &mut a;
//b[0] = 'x';
//c[0] = 'y';
```
