fn main() {
    // Demo data
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8];

    // Use the `map` method
    let squares = numbers.iter().map(|x| x * x);
    for square in squares {
        println!("{}", square);
    }

    // Use the `filter` method
    let evens = numbers.iter().filter(|x| *x % 2 == 0);
    for even in evens {
        println!("{}", even);
    }

}
