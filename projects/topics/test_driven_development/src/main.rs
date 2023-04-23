#[cfg(test)]           // Annotation: the mod is for cargo test
mod tests {            // Define a module named "tests"
    use super::*;      // Use code from the outer module

    #[test]            // Annotation: this function is a test
    fn foo_test() {    // Define a function as usual
      assert!(foo());  // The assert! test macro must be true
    }

}

pub fn foo() -> bool { // Define a function
    true // Always return true
}

fn main() {
    let x = foo();
    println!("{}", x);
}
