# dyn trait in a type position

[Runnable project](/projects/topics/dyn_trait_in_a_type_position)

You can use `&dyn` with a trait name in a type position. This is useful to abstract over a variety of implementations.

Example:

```rust
trait Speak {
    fn speak(&self);
}

type Cat;

impl Speak for Cat {
    fn speak(&self) {
        println!("meow");
    }
}

type Dog;

impl Speak for Dog {
    fn speak(&self) {
        println!("woof");
    }
}

fn main() {
    let pets: Vec<&dyn Speak> = vec![&Cat, &Dog];
    for pet in pets {
        pet.speak();
    }
}
```
