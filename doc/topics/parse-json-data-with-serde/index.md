# Parse JSON data with Serde

Example code to parse JSON data, by using the `serde_json` crate.

```rust
use serde_json::{Result, Value};

fn parse_json(json_string: &str) -> Result<Value> {
    let json: Value = serde_json::from_str(json_string)?;
    Ok(json)
}

fn main() {
    let json_string = r#"
        {
            "name": "John Doe",
            "speaks": ["English", "Mandarin"]
        }
    "#;

    let parsed_json = parse_json(json_string).unwrap();
    let name = parsed_json["name"].as_str().unwrap();
    let languages = parsed_json["speaks"].as_array().unwrap();
    println!("Name: {}", name);
    println!("Speaks: {:?}", languages);
}
```

This code defines a function `parse_json` that takes a JSON string and returns a `serde_json::Value` object. The `serde_json::from_str` function parses the JSON string into a `Value` object. The main function demonstrates how to access the values in the parsed JSON by using the `as_*` methods on the `Value` object. In this example, we access the `name`, `age`, and `speaks` fields of the JSON object and print them to the console. This code assumes that the JSON is well-formed, and matches the expected schema.
