# yew crate - example

[Runnable project](/projects/crates/yew/hello_world)

Example of a web app that says "Hello, World!":

```rust
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ "Hello, World!" }</h1>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
```

This code creates a Yew component that returns HTML, then renders the component to the web page.

