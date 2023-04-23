

// fn trim_period(s: &String) -> &str {
//     s.trim_matches('.')
// }

fn trim_period<'a>(s: &'a String) -> &'a str {
    s.trim_matches('.')
}

// Declare a struct that has lifetime `a` as an input.
// struct MyStruct<'a> {
//     // Declare a string slice that must live at least `a`.
//     my_str: &'a str
// }

// Declare a function that has lifetime `b` as an input,
// an arg that is a MyStruct that must live at least `b`,
// and a return string slice that must lve at least `b`.
// fn foo<'b>(arg: MyStruct<'b>) -> &'b str{
//     println!("{}", arg.my_str);
// }

fn main() {
    let s = String::from("...hello...");
    println!("{}", trim_period(&s))
}
