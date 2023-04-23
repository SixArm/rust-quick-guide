pub trait FizzBuzz {
    fn fizzbuzz(&self) -> String;
}

impl<T> FizzBuzz for T
where 
    T: num::traits::int::PrimInt,
    T: std::fmt::Display,
{
    fn fizzbuzz(&self) -> String {
        let t0 = T::zero();
        let t3 = T::from(3).unwrap();
        let t5 = T::from(5).unwrap();
        match (*self % t3 == t0, *self % t5 == t0) {
            (true, true) => String::from("FizzBuzz"),
            (true, _) => String::from("Fizz"),
            (_, true) => String::from("Buzz"),
            _ => format!("{}", self),
        }
    }
}

fn main() {
    for i in 1..=100 {
        println!("{}", i.fizzbuzz())
    }
}
