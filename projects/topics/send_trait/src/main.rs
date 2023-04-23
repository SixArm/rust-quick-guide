struct Foo {
    x: i32,
    y: String,
}

unsafe impl Send for Foo {}

fn main() {
    let foo = Foo { x: 42, y: "Hello".to_string() };
    std::thread::spawn(move || {
        println!("x = {}, y = {}", foo.x, foo.y);
    }).join().unwrap();
}
