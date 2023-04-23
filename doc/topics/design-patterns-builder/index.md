# Design patterns: builder

The "builder" design pattern creates complex objects via simpler steps. This can be implemented using a struct with setter methods.

```rust
struct Foo {
    a: i32,
    b: i32,
}

struct FooBuilder {
    a: Option<i32>,
    b: Option<i32>,
}

impl FooBuilder {
    fn new() -> Self {
        FooBuilder {
            a: None,
            b: None,
        }
    }

    fn a(mut self, a: i32) -> Self {
        self.a = Some(a); self
    }

    fn b(mut self, b: u32) -> Self {
        self.b = Some(b); self
    }

    fn build(self) -> Foo {
        Foo {
            a: self.a.expect("missing field a"),
            b: self.b.expect("missing field b"),
        }
    }
}

let foo = FooBuilder::new().a(1).b(2).build();
```
