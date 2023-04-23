use rayon::prelude::*;

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let sum = numbers.par_iter().sum::<i32>();
    println!("Sum is {}", sum);
}
