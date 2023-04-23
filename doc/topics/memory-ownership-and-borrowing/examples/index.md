# Memory ownership - example

Example of memory ownership and borrowing in Rust:

```rust
fn main() {
    let mut vec = vec![1, 2, 3];
    print_vec(&vec); // Pass a reference
    vec.push(4); // Modify the vector
    take_vec(vec); // Pass ownership
}

fn print_vec(vec: &Vec<i32>) {
    for num in vec {
        println!("{}", num);
    }
}

fn take_vec(vec: Vec<i32>) {
    println!("Took ownership of {:?}", vec);
}
```

In this example, we define a vector of integers and then pass a reference to the vector to a function called `print_vec`. The `print_vec` function borrows the reference to the vector and iterates over it, printing each element.

Next, we modify the vector by pushing another element onto it, and then pass ownership of the vector to a function called `take_vec`. The `take_vec` function takes ownership of the vector and prints a message to indicate that it has ownership of the vector.

Notice that we use the `&` operator to pass a reference to the vector to `print_vec`. This is an example of borrowing in Rust - we borrow a reference to the vector without taking ownership of it.

In contrast, when we pass the vector to `take_vec`, we don't use the `&` operator. This is an example of taking ownership in Rust - we transfer ownership of the vector to the `take_vec` function.
