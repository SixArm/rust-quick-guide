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
