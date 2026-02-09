use diff_json::{compare_json, DiffFormatter};

fn main() {
    let json1 = r#"{
        "name": "Alice",
        "age": 30,
        "address": {
            "street": "123 Main St",
            "city": "New York"
        },
        "hobbies": ["reading", "swimming"]
    }"#;

    let json2 = r#"{
        "name": "Alice",
        "age": 31,
        "address": {
            "street": "456 Oak Ave",
            "city": "Boston"
        },
        "hobbies": ["reading", "coding"]
    }"#;

    let diffs = compare_json(json1, json2).unwrap();

    let formatter = DiffFormatter::new();

    println!("=== Plain Text Format ===");
    println!("{}", formatter.format(&diffs));

    println!("\n=== Compact Format ===");
    println!("{}", formatter.format_compact(&diffs));

    println!("\n=== JSON Format ===");
    println!("{}", formatter.format_json(&diffs));
}
