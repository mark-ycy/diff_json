#[cfg(test)]
mod tests {
    use crate::{compare_json, compare_values, DiffFormatter, JsonDiff};
    use serde_json::json;

    #[test]
    fn test_identical_objects() {
        let json1 = r#"{"name": "Alice", "age": 30}"#;
        let json2 = r#"{"name": "Alice", "age": 30}"#;

        let diffs = compare_json(json1, json2).unwrap();
        assert!(diffs.is_empty());
    }

    #[test]
    fn test_added_field() {
        let json1 = r#"{"name": "Alice"}"#;
        let json2 = r#"{"name": "Alice", "age": 30}"#;

        let diffs = compare_json(json1, json2).unwrap();
        assert_eq!(diffs.len(), 1);
        assert_eq!(diffs[0].path, "age");
        assert_eq!(format!("{:?}", diffs[0].diff_type), "Added");
    }

    #[test]
    fn test_removed_field() {
        let json1 = r#"{"name": "Alice", "age": 30}"#;
        let json2 = r#"{"name": "Alice"}"#;

        let diffs = compare_json(json1, json2).unwrap();
        assert_eq!(diffs.len(), 1);
        assert_eq!(diffs[0].path, "age");
        assert_eq!(format!("{:?}", diffs[0].diff_type), "Removed");
    }

    #[test]
    fn test_modified_field() {
        let json1 = r#"{"name": "Alice", "age": 30}"#;
        let json2 = r#"{"name": "Alice", "age": 31}"#;

        let diffs = compare_json(json1, json2).unwrap();
        assert_eq!(diffs.len(), 1);
        assert_eq!(diffs[0].path, "age");
        assert_eq!(format!("{:?}", diffs[0].diff_type), "Modified");
    }

    #[test]
    fn test_nested_objects() {
        let json1 = r#"{"user": {"name": "Alice", "age": 30}}"#;
        let json2 = r#"{"user": {"name": "Alice", "age": 31}}"#;

        let diffs = compare_json(json1, json2).unwrap();
        assert_eq!(diffs.len(), 1);
        assert_eq!(diffs[0].path, "user.age");
    }

    #[test]
    fn test_arrays() {
        let v1 = json!([1, 2, 3]);
        let v2 = json!([1, 2, 4]);

        let diffs = compare_values(&v1, &v2);
        assert_eq!(diffs.len(), 1);
        assert_eq!(diffs[0].path, "[2]");
    }

    #[test]
    fn test_array_added_element() {
        let v1 = json!([1, 2, 3]);
        let v2 = json!([1, 2, 3, 4]);

        let diffs = compare_values(&v1, &v2);
        assert_eq!(diffs.len(), 1);
        assert_eq!(diffs[0].path, "[3]");
        assert_eq!(format!("{:?}", diffs[0].diff_type), "Added");
    }

    #[test]
    fn test_array_removed_element() {
        let v1 = json!([1, 2, 3, 4]);
        let v2 = json!([1, 2, 3]);

        let diffs = compare_values(&v1, &v2);
        assert_eq!(diffs.len(), 1);
        assert_eq!(diffs[0].path, "[3]");
        assert_eq!(format!("{:?}", diffs[0].diff_type), "Removed");
    }

    #[test]
    fn test_arrays_ignore_order() {
        let v1 = json!([1, 2, 3]);
        let v2 = json!([3, 2, 1]);

        let differ = JsonDiff::new().ignore_order(true);
        let diffs = differ.diff(&v1, &v2);
        assert!(diffs.is_empty());
    }

    #[test]
    fn test_arrays_preserve_order() {
        let v1 = json!([1, 2, 3]);
        let v2 = json!([3, 2, 1]);

        let differ = JsonDiff::new().ignore_order(false);
        let diffs = differ.diff(&v1, &v2);
        assert!(!diffs.is_empty());
    }

    #[test]
    fn test_complex_json() {
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
        assert!(diffs.len() >= 3);
    }

    #[test]
    fn test_formatter_format() {
        let v1 = json!({"name": "Alice", "age": 30});
        let v2 = json!({"name": "Bob", "age": 31});

        let diffs = compare_values(&v1, &v2);
        let formatter = DiffFormatter::new();
        let output = formatter.format(&diffs);

        assert!(output.contains("name"));
        assert!(output.contains("age"));
    }

    #[test]
    fn test_formatter_compact() {
        let v1 = json!({"name": "Alice"});
        let v2 = json!({"name": "Bob"});

        let diffs = compare_values(&v1, &v2);
        let formatter = DiffFormatter::new();
        let output = formatter.format_compact(&diffs);

        assert!(output.contains("Found 1 difference"));
    }

    #[test]
    fn test_formatter_json() {
        let v1 = json!({"name": "Alice"});
        let v2 = json!({"name": "Bob"});

        let diffs = compare_values(&v1, &v2);
        let formatter = DiffFormatter::new();
        let output = formatter.format_json(&diffs);

        assert!(output.contains("diffs"));
        assert!(output.contains("summary"));
    }

    #[test]
    fn test_null_values() {
        let v1 = json!(null);
        let v2 = json!(null);

        let diffs = compare_values(&v1, &v2);
        assert!(diffs.is_empty());
    }

    #[test]
    fn test_boolean_values() {
        let v1 = json!(true);
        let v2 = json!(false);

        let diffs = compare_values(&v1, &v2);
        assert_eq!(diffs.len(), 1);
        assert_eq!(format!("{:?}", diffs[0].diff_type), "Modified");
    }

    #[test]
    fn test_empty_objects() {
        let v1 = json!({});
        let v2 = json!({});

        let diffs = compare_values(&v1, &v2);
        assert!(diffs.is_empty());
    }

    #[test]
    fn test_empty_arrays() {
        let v1 = json!([]);
        let v2 = json!([]);

        let diffs = compare_values(&v1, &v2);
        assert!(diffs.is_empty());
    }

    #[test]
    fn test_invalid_json() {
        let json1 = r#"{"name": "Alice"}"#;
        let json2 = r#"invalid json"#;

        let result = compare_json(json1, json2);
        assert!(result.is_err());
    }
}
