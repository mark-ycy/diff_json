use serde_json::Value;

pub mod diff;
pub mod formatter;

#[cfg(test)]
mod tests;

pub use diff::{Diff, DiffType, JsonDiff};
pub use formatter::DiffFormatter;

pub fn compare_json(json1: &str, json2: &str) -> Result<Vec<Diff>, String> {
    let v1: Value = serde_json::from_str(json1).map_err(|e| e.to_string())?;
    let v2: Value = serde_json::from_str(json2).map_err(|e| e.to_string())?;

    let differ = JsonDiff::new();
    Ok(differ.diff(&v1, &v2))
}

pub fn compare_values(v1: &Value, v2: &Value) -> Vec<Diff> {
    let differ = JsonDiff::new();
    differ.diff(v1, v2)
}
