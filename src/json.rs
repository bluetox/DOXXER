pub fn parse_json(data: &str, field: &str) -> String {
    // Parsing JSON data
    let json_data: serde_json::Value = match serde_json::from_str(data) {
        Ok(value) => value,
        Err(err) => {
            println!("Error parsing JSON: {}", err);
            // Return a default value or panic here if desired
            return String::new();
        }
    };

    // Extracting the specified field from the JSON
    if let Some(field_value) = json_data.get(field) {
        if let Some(value_str) = field_value.as_str() {
            return value_str.to_string();
        } else {
            println!("Field '{}' found in JSON, but it's not a string", field);
            // Return a default value or panic here if desired
            return String::new();
        }
    } else {
        println!("Couldn't find field '{}'", field);
        // Return a default value or panic here if desired
        return String::new();
    }
}
