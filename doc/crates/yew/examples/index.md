# yew crate - example

[Runnable project](/projects/crates/yew)

Exampel of a simple website:

```rust
#[macro_use]
extern crate yew;

use yew::prelude::*;

struct Model { /* Your model's data here. */ }

enum Msg { /* Your message types here. */ }

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>)
    -> Self {
        // Your create hook implementation here.
        Model { /* Initialize your model data here. */ }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        // Your message update implementation here.
        true // Update the component always.
    }

    fn view(&self) -> Html {
        // Your HTML template and rendering implementation here.
        html! { <div>{ "Hello" }</div> }
    }
}

fn main() {
    yew::start_app::<Model>();
}
```

This code creates a Yew component, and renders a "Hello" message on the web page.