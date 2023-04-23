use std::sync::Arc;

struct MyStruct {
    x: i32,
}

impl MyStruct {
    fn my_function(&self) {
        println!("x is {}", self.x);
    }
}

fn main() {
    let s = MyStruct { x: 1 };
    let shared = Arc::new(s);
    std::thread::spawn({
        let shared = shared.clone();
        move || { shared.my_function(); }
    }).join().unwrap();
}
