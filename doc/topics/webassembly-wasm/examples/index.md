# WebAssembly - example

Create a new Rust project, such as running:

```sh
cargo new wasm-example --lib
```

Add the `wasm-bindgen` dependency to your `Cargo.toml` file.

In your `lib.rs` file, add the `wasm_bindgen` macro to the top of the file, and define a simple Rust function that takes two numbers and returns their sum:

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

Build your Rust code as a WebAssembly module by running the following command, which creates a WASM file called `wasm-example.wasm` in the `target/wasm32-unknown-unknown/release/` directory:


```sh
cargo +nightly build --target wasm32-unknown-unknown --release
```

Finally, create a JavaScript file that loads the WASM module and calls your Rust function:

```javascript
import("./wasm_example_bg.wasm").then((module) => {
  const { add } = module;
  console.log(add(1, 2)); // outputs 3
});
```

This JavaScript code loads the WASM module using the `import()` function, which is a new feature in JavaScript that allows you to dynamically load modules at runtime. Once the module is loaded, you can call your Rust function using the `add` variable.
