use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // The `write!` macro writes fields into the formatter
        write!(f, "x is {} and y is {}", self.x, self.y)
    }
}

fn main() {
    let p = Point { x: 1, y: 2 };
    println!("Point {}", p); // "Point x is 1 and y is 2"
}
