# Design patterns: adapter

The "adapter" structural design pattern enables incompatible interfaces to collaborate. This can be implemented using a adapter struct that wraps an adaptee struct.

```rust
// Suppose we have a typical struct that we want to adapt.
// This struct is typically know as the "adaptee".
struct CircleWithRadius {
    radius: f32;
}

// The adapter structural design pattern typically means
// we define an outer struct that wraps an inner struct.
// The outer struct is typically known as the "adapter".
// The inner struct is typically known as the "adaptee".
struct CircleWithDiameter {
    adaptee: CircleWithRadius;
}

// We implement the adapter methods, such as these accessors,
// so the methods actually get and set the adaptee's data.
// This is similar to a proxy object, or to a facade object.
impl CircleWithDiameter {
    fn diameter(&self) -> f32 {
        adaptee.radius * 2;
    }

    fn set_diameter(&self, diameter: f32) {
        adaptee.radius = diameter / 2;
    }
}
```
