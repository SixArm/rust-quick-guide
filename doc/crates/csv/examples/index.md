# CSV crate - example

[Runnable project](/projects/crates/csv/read_a_spreadsheet_file)

Example to read a spreadsheet file then print the records:

```rust
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use csv::ReaderBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "spreadsheet.csv";
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .from_reader(contents.as_bytes());
    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}
```

Notably, this example skips error handling. If you are writing production-quality code, then you would want to add code for error handling, and potentially also for error correction.
