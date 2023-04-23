# Liskov substitution principle - example

Example:

```rust
trait Drawable {
    fn draw(&self);
}

fn draw_anything(drawable: &dyn Drawable) {
    drawable.draw();
}

struct Circle {
    radius: i32,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Circle with radius {}", self.radius);
    }
}

fn main() {
    let circle = Circle { radius: 1 };
    draw_anything(&circle);
}
```

This defines a struct `Circle` and implements the `Drawable` trait. The `draw_anything` function takes any object that implements the `Drawable` interface, which means that it can accept circles or anything else that implements `Drawable`. interface. The function is an example of Liskov substitution principle in action, because any `Drawable` can be given   .
