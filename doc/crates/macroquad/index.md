# macroquad crate for simple games

<https://crates.io/crates/macroquad>

The Rust macroquad crate is a simple and easy to use game library for Rust programming language.

macroquad attempts to avoid any rust-specific programming concepts like lifetimes/borrowing, making it very friendly for rust beginners.

Features:

* Same code for all supported platforms, no platform dependent defines required
* Efficient 2D rendering with automatic geometry batching
* Minimal amount of dependencies: build after cargo clean takes only 16s on x230(~6years old laptop)
* Immediate mode UI library included
* Single command deploy for both WASM and Android build instructions

Example:

```rust
use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    loop {
        clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_text("HELLO", 20.0, 20.0, 20.0, DARKGRAY);

        next_frame().await
    }
}
```
