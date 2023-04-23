use once_cell_regex::regex;

fn main() {
    let r = regex!("hello");
    let x = r.is_match("hello world");
    println!("{}", x); // prints "true"
}
