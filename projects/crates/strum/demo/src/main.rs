use strum_macros::EnumString;

#[derive(Debug, EnumString)]
enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    let color = "Red".parse::<Color>().unwrap();
    println!("{:?}", color);
}
