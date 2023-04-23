#[allow(unused_assignments, unused_variables)]

fn increment_with_pass_by_value(mut num: i32) {
    num += 1;
}

fn increment_with_pass_by_reference(num: &mut i32) {
    *num += 1;
}

fn main() {
    let mut x = 1;
    increment_with_pass_by_value(x);
    println!("x is {}", x); // x is still 1
    increment_with_pass_by_reference(&mut x);
    println!("x is {}", x); // x is now 2
}
