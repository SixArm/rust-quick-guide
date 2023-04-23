# Strum crate for enums

The Rust Strum crate provides macros for working with enums. The crate provides several useful macros that can be used to derive implementations for various traits on enums.

* EnumString - Derives the ability to parse strings into enum variants using the FromStr trait.

* EnumVariantNames - Derives a method that returns a list of the enum's variant names as strings.

* Display - Derives the ability to convert enum variants to strings using the Display trait.

* AsRefStr - Derives the ability to convert enum variants to string slices using the AsRef trait.

Example:

```rust
use strum_macros::EnumString;

#[derive(Debug, EnumString)]
enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    let color = "Red".parse::<Color>().unwrap();
    println!("{:?}", color);
}
```

This example uses the `EnumString` macro to derive the `FromStr` trait for the `Color` enum. This enables us to parse the string "Red". The `unwrap()` method handles any parse errors.
