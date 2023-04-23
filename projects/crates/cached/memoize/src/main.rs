use cached::proc_macro::cached;

#[cached]
fn fibonacci(n: usize) -> usize {
    if n == 0 || n == 1 { return n }
    fibonacci(n-1) + fibonacci(n-2)
}

fn main() {
    println!("{}", fibonacci(12));
}
