use textwrap::{fill, wrap};

fn main() {
    let input_text = "Rust is a great programming language for us";
    println!("{}", fill(input_text, 22));
    println!("{:?}", wrap(input_text, 22));
}
