# Borrow splitting

Borrow splitting, a.k.a. partial borrowing, is when you try to borrow in multiple ways that can interfere with each other.

This example fails to compile because of borrow splitting:

```rust
// Create a typical struct
struct Foo {
    a: i32,
    b: i32,
}

// Create mutabale accessors
impl Foo {
    pub fn a_mut(&mut self) -> &mut i32 {
        &mut self.a
    }
    pub fn b_mut(&mut self) -> &mut i32 {
        &mut self.b
    }
}

// Compile succeeds because `a` and `b` are independent
pub fn increment(a: &mut i32, b: &mut i32) {
    *a = *a + 1;
    *b = *b + 1;
}

// Compile error because `a` and `b` are borrow splitting:
// cannot borrow `*self` as mutable more than once at a time
impl Foo {
    pub fn increment(&mut self) {
        let a = self.a_mut();
        let b = self.b_mut();
        *a = *a + 1;
        *b = *b + 1;
    }
}
```
