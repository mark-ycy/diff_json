use crate::diff::{Diff, DiffType};

pub struct DiffFormatter {
    indent: String,
    show_values: bool,
}

impl DiffFormatter {
    pub fn new() -> Self {
        Self {
            indent: "  ".to_string(),
            show_values: true,
        }
    }

    pub fn indent(mut self, indent: &str) -> Self {
        self.indent = indent.to_string();
        self
    }

    pub fn show_values(mut self, show: bool) -> Self {
        self.show_values = show;
        self
    }

    pub fn format(&self, diffs: &[Diff]) -> String {
        let mut output = String::new();

        for diff in diffs {
            let prefix = match diff.diff_type {
                DiffType::Added => "+",
                DiffType::Removed => "-",
                DiffType::Modified => "~",
                DiffType::Moved => ">",
            };

            output.push_str(&format!("{} {}: ", prefix, diff.path));

            if self.show_values {
                match &diff.diff_type {
                    DiffType::Added => {
                        if let Some(value) = &diff.new_value {
                            output.push_str(&format!("{:?}", value));
                        }
                    }
                    DiffType::Removed => {
                        if let Some(value) = &diff.old_value {
                            output.push_str(&format!("{:?}", value));
                        }
                    }
                    DiffType::Modified => {
                        output.push_str(&format!("{:?} -> {:?}", diff.old_value, diff.new_value));
                    }
                    DiffType::Moved => {
                        if let Some(value) = &diff.new_value {
                            output.push_str(&format!("{:?}", value));
                        }
                    }
                }
            }

            output.push('\n');
        }

        output
    }

    pub fn format_json(&self, diffs: &[Diff]) -> String {
        let mut result = serde_json::json!({
            "diffs": [],
            "summary": {
                "total": diffs.len(),
                "added": 0,
                "removed": 0,
                "modified": 0,
                "moved": 0
            }
        });

        let mut summary = result["summary"].clone();

        for diff in diffs {
            let diff_json = serde_json::json!({
                "path": diff.path,
                "type": format!("{:?}", diff.diff_type),
                "old_value": diff.old_value,
                "new_value": diff.new_value
            });

            result["diffs"].as_array_mut().unwrap().push(diff_json);

            match diff.diff_type {
                DiffType::Added => {
                    summary["added"] = (summary["added"].as_i64().unwrap() + 1).into();
                }
                DiffType::Removed => {
                    summary["removed"] = (summary["removed"].as_i64().unwrap() + 1).into();
                }
                DiffType::Modified => {
                    summary["modified"] = (summary["modified"].as_i64().unwrap() + 1).into();
                }
                DiffType::Moved => {
                    summary["moved"] = (summary["moved"].as_i64().unwrap() + 1).into();
                }
            }
        }

        result["summary"] = summary;

        serde_json::to_string_pretty(&result).unwrap_or_default()
    }

    pub fn format_compact(&self, diffs: &[Diff]) -> String {
        if diffs.is_empty() {
            return "No differences found".to_string();
        }

        let mut output = String::new();
        output.push_str(&format!("Found {} difference(s):\n", diffs.len()));

        for (i, diff) in diffs.iter().enumerate() {
            output.push_str(&format!("{}. {}\n", i + 1, diff));
        }

        output
    }

    pub fn format_colored(&self, diffs: &[Diff]) -> String {
        let mut output = String::new();

        for diff in diffs {
            let (color_name, symbol) = match diff.diff_type {
                DiffType::Added => ("green", "+"),
                DiffType::Removed => ("red", "-"),
                DiffType::Modified => ("yellow", "~"),
                DiffType::Moved => ("cyan", ">"),
            };

            output.push_str(&format!("{} [{}]: ", symbol, color_name));
            output.push_str(&diff.path);
            output.push(' ');

            if self.show_values {
                match &diff.diff_type {
                    DiffType::Added => {
                        if let Some(value) = &diff.new_value {
                            output.push_str(&format!("{:?}", value));
                        }
                    }
                    DiffType::Removed => {
                        if let Some(value) = &diff.old_value {
                            output.push_str(&format!("{:?}", value));
                        }
                    }
                    DiffType::Modified => {
                        output.push_str(&format!("{:?} -> {:?}", diff.old_value, diff.new_value));
                    }
                    DiffType::Moved => {
                        if let Some(value) = &diff.new_value {
                            output.push_str(&format!("{:?}", value));
                        }
                    }
                }
            }

            output.push('\n');
        }

        output
    }
}

impl Default for DiffFormatter {
    fn default() -> Self {
        Self::new()
    }
}
