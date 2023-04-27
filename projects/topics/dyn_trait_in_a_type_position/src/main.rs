trait Speak {
    fn speak(&self);
}

struct Cat;

impl Speak for Cat {
    fn speak(&self) {
        println!("meow");
    }
}

struct Dog;

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
