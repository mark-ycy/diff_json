# diff_json

A powerful and flexible JSON diff library for Rust that helps you compare two JSON values and identify differences.

## Features

- **Comprehensive Diff Detection**: Detects added, removed, and modified fields in JSON objects
- **Array Comparison**: Compare arrays with optional order-insensitive matching
- **Nested Structure Support**: Handles deeply nested JSON objects and arrays
- **Multiple Output Formats**: Format diffs as plain text, JSON, or compact summaries
- **Easy-to-Use API**: Simple and intuitive API for quick integration
- **Zero Dependencies**: Minimal external dependencies (only serde and serde_json)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
diff_json = "0.1.0"
```

## Usage

### Basic Example

```rust
use diff_json::{compare_json, DiffFormatter};

let json1 = r#"{"name": "Alice", "age": 30}"#;
let json2 = r#"{"name": "Alice", "age": 31}"#;

let diffs = compare_json(json1, json2).unwrap();

let formatter = DiffFormatter::new();
println!("{}", formatter.format(&diffs));
```

### Working with Values

```rust
use diff_json::{compare_values, JsonDiff};
use serde_json::json;

let v1 = json!({"name": "Alice", "age": 30});
let v2 = json!({"name": "Bob", "age": 31});

let diffs = compare_values(&v1, &v2);
for diff in diffs {
    println!("{}", diff);
}
```

### Array Comparison with Order Ignored

```rust
use diff_json::JsonDiff;
use serde_json::json;

let v1 = json!([1, 2, 3]);
let v2 = json!([3, 2, 1]);

let differ = JsonDiff::new().ignore_order(true);
let diffs = differ.diff(&v1, &v2);
// diffs will be empty since arrays contain the same elements
```

### Different Output Formats

```rust
use diff_json::{compare_values, DiffFormatter};
use serde_json::json;

let v1 = json!({"name": "Alice"});
let v2 = json!({"name": "Bob"});

let diffs = compare_values(&v1, &v2);
let formatter = DiffFormatter::new();

// Plain text format
println!("{}", formatter.format(&diffs));

// Compact format
println!("{}", formatter.format_compact(&diffs));

// JSON format
println!("{}", formatter.format_json(&diffs));
```

## API Reference

### `compare_json(json1: &str, json2: &str) -> Result<Vec<Diff>, String>`

Compare two JSON strings and return a list of differences.

### `compare_values(v1: &Value, v2: &Value) -> Vec<Diff>`

Compare two `serde_json::Value` instances and return a list of differences.

### `JsonDiff`

Main diff engine with configurable options.

- `new()`: Create a new `JsonDiff` instance
- `ignore_order(bool)`: Set whether array comparison should ignore order
- `diff(v1: &Value, v2: &Value) -> Vec<Diff>`: Compare two values

### `DiffFormatter`

Format diff results in various styles.

- `new()`: Create a new formatter
- `indent(&str)`: Set indentation for formatted output
- `show_values(bool)`: Control whether to show values in output
- `format(&[Diff]) -> String`: Format diffs as plain text
- `format_compact(&[Diff]) -> String`: Format diffs in compact style
- `format_json(&[Diff]) -> String`: Format diffs as JSON
- `format_colored(&[Diff]) -> String`: Format diffs with color indicators

### `Diff`

Represents a single difference between two JSON values.

- `path: String`: JSON path to the changed element
- `diff_type: DiffType`: Type of change (Added, Removed, Modified, Moved)
- `old_value: Option<Value>`: Original value (if applicable)
- `new_value: Option<Value>`: New value (if applicable)

## License

This project is licensed under either of:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Examples

See the `examples` directory for more usage examples.

## Testing

Run tests with:

```bash
cargo test
```

## License

MIT OR Apache-2.0
