# Design patterns: introduction

Design patterns in programming refer to reusable solutions to common problems that arise during software development. These patterns provide a standard set of practices, templates and a recommended course of action for solving recurring problems. They are proven solutions that help developers to build software that is more modular, maintainable, and scalable.

Design patterns are used by software developers to ensure that the code follows best practices while tackling common problems. They are grouped into three categories: creational, structural, and behavioral.

Creational patterns are used to create objects and instances of classes during runtime. Structural patterns are aimed at developing the overall structure of the code, while behavioral patterns are used to manage communication between object instances.

Using a well-defined design pattern allows developers to focus on the software's functionalities rather than the design aspects of the code. Some of the commonly used design patterns in programming include Singleton, Observer, Decorator, Facade, Adapter, Iterator, Builder, and many more.

As one example, the Iterator design pattern provides a way to iterate over a collection of objects. In Rust, this is built into the language with the Iterator trait.

Example:

```rust
let numbers = vec![1, 2, 3, 4, 5];
for number in numbers.iter() {
    println!("{}", number);
}
```
