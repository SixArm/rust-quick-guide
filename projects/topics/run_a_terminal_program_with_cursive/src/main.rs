use cursive::{Cursive, CursiveExt};
use cursive::views::{Dialog, TextView};

fn main() {
    let mut siv = Cursive::default();

    siv.add_layer(
        Dialog::around(TextView::new("Hello, world!"))
            .title("Cursive Example")
            .button("Quit", |s| s.quit()),
    );

    siv.run();
    println!("Ok")
}
