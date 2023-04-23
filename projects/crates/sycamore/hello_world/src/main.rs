use sycamore::prelude::*;

#[component]
fn Hello<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        p { "Hello World!" }
    }
}

fn main() {
    sycamore::render(|cx| {
        view! { cx, Hello {} }
    });
}
