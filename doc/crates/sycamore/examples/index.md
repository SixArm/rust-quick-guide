# Sycamore crate - example

[Runnable project](/projects/crates/sycamore/hello_world)

You may want/need to add a WASM target:

```sh
rustup target add wasm32-unknown-unknown
```

Edit file `main.rs` which will contain your website code:

```rust
use sycamore::prelude::*;

#[component]
fn Hello<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        p { "Hello, World!" }
    }
}

fn main() {
    sycamore::render(|cx| {
        view! { cx, Hello {} }
    });
}
```

Create top-level file `index.html`:

```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8" />
    <meta http-equiv="X-UA-Compatible" content="IE=edge" />
    <title>My Title</title>
    <meta name="viewport" content="width=device-width, initial-scale=1" />
  </head>
  <body></body>
</html>
```

Serve the website by using the `trunk` build tool:

```sh
trunk serve
```

Browse <http://localhost:8000>
