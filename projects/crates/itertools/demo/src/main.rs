use itertools::{Itertools, join};

fn main() {
    // Demo data
    let numbers = vec![1, 2, 3];
    let letters = vec!['a', 'b', 'c'];

    for (n, l) in numbers.iter().cartesian_product(letters.iter()) {
        println!("{}{}", n, l);
    }
    
    for p in letters.iter().permutations(2) {
        println!("{:?}", p);
    }

    let joined = join(letters, ", ");
    println!("{:?}", joined);
}
