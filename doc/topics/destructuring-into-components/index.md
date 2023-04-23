# Destructuring into components

In Rust, destructuring is the process of taking apart a complex data structure (such as a tuple, struct, or enum) into its individual components.

Destructuring a tuple:

```rust
let my_tuple = (1, 2);
let (a, b) = my_tuple; // Assign a = 1, b = 2
```

Destructuring struct fields:

```rust
struct MyStruct {
    x: i32,
    y: String,
}

let my_struct = MyStruct { x: 42, y: String::from("hello") };
let MyStruct { a, b } = my_struct; // Assign a = 42, b = "hello"
```

Destructuring an enum variant:

```rust
enum MyEnum {
    Variant1(i32),
    Variant2(String),
}

let my_enum = MyEnum::Variant1(42);
match my_enum {
    MyEnum::Variant1(n) => println!("Got a number: {}", n),
    MyEnum::Variant2(s) => println!("Got a string: {}", s),
}
```
