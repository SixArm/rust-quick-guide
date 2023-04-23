use regex::Regex;
use once_cell::sync::Lazy;

fn main() {
    static RE: Lazy<Regex> =
        Lazy::new(|| Regex::new("hello").unwrap());
    let matched = RE.is_match("hello world");
    println!("{}", matched);
}
