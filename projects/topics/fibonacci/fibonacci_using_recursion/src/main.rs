pub fn fibonacci(n: i32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        2.. => fibonacci(n - 1) + fibonacci(n - 2),
        _ => panic!("{}", n)
    }
}

fn main() {
    println!("{}", fibonacci(12)); // print 144
}
