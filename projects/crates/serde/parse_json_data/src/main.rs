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
