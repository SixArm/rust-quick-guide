use std::borrow::Cow;

fn main() {
    let a = ['a', 'b', 'c'];
    let mut b = Cow::Borrowed(&a);

    // Here the `b` Cow enum is borrowed, and points to `a`.
    match b {
        Cow::Borrowed(_) => println!("Borrowed"),
        Cow::Owned(_) => println!("Owned"),
    }

    // Tell `b` to be mutable i.e. to clone, then change a letter.
    b.to_mut()[0] = 'x';

    // Here the `b` Cow enum is owned, and points its own data.
    match b {
        Cow::Borrowed(_) => println!("Borrow"),
        Cow::Owned(_) => println!("Owned"),
    }
}
