# Search text file lines with regex

Example to search text file lines by using the `regex` crate for regular expression pattern matching.

```rust
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

fn main() {
    // Regular expression: word break + "in" + word break
    let regex = Regex::new(r"\bin\b").expect("regex");

    // Open an existing file and prepare to read its lines
    let file = File::open("example.txt").expect("file");
    let lines = io::BufReader::new(file).lines();

    // For each line, try the regex
    for line in lines {
        if let Ok(string) = line {
            // If the string matches, then print it
            if regex.is_match(&string) {
                println!("match: {}", string);
            }
        }
    }
}
```

The example opens a text file, then reads each line. The example loop tries the regex `is_match` method on each line's string. If the string matches, then the function prints it.

