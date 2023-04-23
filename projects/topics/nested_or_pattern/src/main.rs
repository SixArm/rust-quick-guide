
fn main() {
    let value = Some(7);

    // If-let statements ...
    
    // ... without nested or-pattern
    if let Some(2) | Some(3) | Some(5) | Some(7) = value {
        println!("prime");
    }

    // ... with nested or-pattern
    if let Some(2 | 3 | 5 | 7) = value {
        println!("prime");
    }

    // Match statements...

    // ... without nested-or-pattern
    match value {
        Some(2) | Some(3) | Some(5) | Some(7) => println!("prime"),
        Some(0) | Some(1) | Some(4) | Some(9) => println!("square"),
        None => println!("nothing"),
        _ => println!("something else"),
    }

    // Match statement with nested-or-pattern
    match value {
        Some(2 | 3 | 5 | 7) => println!("prime"),
        Some(0 | 1 | 4 | 9) => println!("square"),
        None => println!("nothing"),
        Some(n) => println!("{} is something else", n),
    }

    // Match statement with nested-or-pattern and @-bindng
    match value {
        Some(n @ (2 | 3 | 5 | 7)) => println!("{n} is a prime"),
        Some(n @ (0 | 1 | 4 | 9)) => println!("{n} is a square"),
        None => println!("nothing"),
        Some(n) => println!("{} is something else", n),
    }

}
