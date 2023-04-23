pub mod outer {
    pub mod inner {
        pub fn hello() {
            println!("hello");
        }
    }
}

use outer::inner::hello;
fn main() {
    hello()
}
