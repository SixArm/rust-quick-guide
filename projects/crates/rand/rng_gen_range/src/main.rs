use rand::Rng;

fn main() {
    // Create a random number generator
    let mut rng = rand::thread_rng();
    let number = rng.gen_range(1..=100);
    println!("{}", number);
}
