use std::pin::Pin;
struct Data {
    value: i32,
}
impl Data {
    fn new(value: i32) -> Self {
        Self { value }
    }
}
fn main() {
    let data = Data::new(1);
    let pinned_data = Pin::new(&data);

    // Invalid move of `data`:
    // let moved_data = data;

    // Invalid move of `pinned_data`:
    // let moved_pinned_data = pinned_data;

    // We can access the value of `data` through `pinned_data`
    println!("{}", pinned_data.value);
}
