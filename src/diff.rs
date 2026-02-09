use serde_json::Value;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum DiffType {
    Added,
    Removed,
    Modified,
    Moved,
}

#[derive(Debug, Clone)]
pub struct Diff {
    pub path: String,
    pub diff_type: DiffType,
    pub old_value: Option<Value>,
    pub new_value: Option<Value>,
}

impl fmt::Display for Diff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.diff_type {
            DiffType::Added => {
                write!(f, "Added at '{}': {:?}", self.path, self.new_value)
            }
            DiffType::Removed => {
                write!(f, "Removed from '{}': {:?}", self.path, self.old_value)
            }
            DiffType::Modified => {
                write!(
                    f,
                    "Modified at '{}': {:?} -> {:?}",
                    self.path, self.old_value, self.new_value
                )
            }
            DiffType::Moved => {
                write!(
                    f,
                    "Moved: {} -> {}",
                    self.path,
                    self.new_value.as_ref().unwrap()
                )
            }
        }
    }
}

pub struct JsonDiff {
    ignore_order: bool,
}

impl JsonDiff {
    pub fn new() -> Self {
        Self {
            ignore_order: false,
        }
    }

    pub fn ignore_order(mut self, ignore: bool) -> Self {
        self.ignore_order = ignore;
        self
    }

    pub fn diff(&self, v1: &Value, v2: &Value) -> Vec<Diff> {
        let mut diffs = Vec::new();
        self.diff_values(v1, v2, "", &mut diffs);
        diffs
    }

    fn diff_values(&self, v1: &Value, v2: &Value, path: &str, diffs: &mut Vec<Diff>) {
        match (v1, v2) {
            (Value::Null, Value::Null) => {}
            (Value::Bool(b1), Value::Bool(b2)) if b1 == b2 => {}
            (Value::Number(n1), Value::Number(n2)) if n1 == n2 => {}
            (Value::String(s1), Value::String(s2)) if s1 == s2 => {}
            (Value::Array(a1), Value::Array(a2)) => {
                self.diff_arrays(a1, a2, path, diffs);
            }
            (Value::Object(o1), Value::Object(o2)) => {
                self.diff_objects(o1, o2, path, diffs);
            }
            _ => {
                diffs.push(Diff {
                    path: path.to_string(),
                    diff_type: DiffType::Modified,
                    old_value: Some(v1.clone()),
                    new_value: Some(v2.clone()),
                });
            }
        }
    }

    fn diff_arrays(&self, a1: &[Value], a2: &[Value], path: &str, diffs: &mut Vec<Diff>) {
        if self.ignore_order {
            self.diff_arrays_ignore_order(a1, a2, path, diffs);
        } else {
            self.diff_arrays_preserve_order(a1, a2, path, diffs);
        }
    }

    fn diff_arrays_preserve_order(
        &self,
        a1: &[Value],
        a2: &[Value],
        path: &str,
        diffs: &mut Vec<Diff>,
    ) {
        let max_len = a1.len().max(a2.len());

        for i in 0..max_len {
            let new_path = if path.is_empty() {
                format!("[{}]", i)
            } else {
                format!("{}[{}]", path, i)
            };

            match (a1.get(i), a2.get(i)) {
                (Some(v1), Some(v2)) => {
                    self.diff_values(v1, v2, &new_path, diffs);
                }
                (Some(v1), None) => {
                    diffs.push(Diff {
                        path: new_path,
                        diff_type: DiffType::Removed,
                        old_value: Some(v1.clone()),
                        new_value: None,
                    });
                }
                (None, Some(v2)) => {
                    diffs.push(Diff {
                        path: new_path,
                        diff_type: DiffType::Added,
                        old_value: None,
                        new_value: Some(v2.clone()),
                    });
                }
                (None, None) => {}
            }
        }
    }

    fn diff_arrays_ignore_order(
        &self,
        a1: &[Value],
        a2: &[Value],
        path: &str,
        diffs: &mut Vec<Diff>,
    ) {
        let mut unused1: Vec<bool> = vec![false; a1.len()];
        let mut unused2: Vec<bool> = vec![false; a2.len()];

        for (i, v1) in a1.iter().enumerate() {
            let mut found = false;
            for (j, v2) in a2.iter().enumerate() {
                if !unused2[j] && self.values_equal(v1, v2) {
                    unused1[i] = true;
                    unused2[j] = true;
                    found = true;
                    break;
                }
            }

            if !found {
                let new_path = if path.is_empty() {
                    format!("[{}]", i)
                } else {
                    format!("{}[{}]", path, i)
                };

                diffs.push(Diff {
                    path: new_path,
                    diff_type: DiffType::Removed,
                    old_value: Some(v1.clone()),
                    new_value: None,
                });
            }
        }

        for (j, v2) in a2.iter().enumerate() {
            if !unused2[j] {
                let new_path = if path.is_empty() {
                    format!("[{}]", j)
                } else {
                    format!("{}[{}]", path, j)
                };

                diffs.push(Diff {
                    path: new_path,
                    diff_type: DiffType::Added,
                    old_value: None,
                    new_value: Some(v2.clone()),
                });
            }
        }
    }

    fn diff_objects(
        &self,
        o1: &serde_json::Map<String, Value>,
        o2: &serde_json::Map<String, Value>,
        path: &str,
        diffs: &mut Vec<Diff>,
    ) {
        let all_keys: std::collections::HashSet<&String> = o1.keys().chain(o2.keys()).collect();

        for key in all_keys {
            let new_path = if path.is_empty() {
                key.clone()
            } else {
                format!("{}.{}", path, key)
            };

            match (o1.get(key), o2.get(key)) {
                (Some(v1), Some(v2)) => {
                    self.diff_values(v1, v2, &new_path, diffs);
                }
                (Some(v1), None) => {
                    diffs.push(Diff {
                        path: new_path,
                        diff_type: DiffType::Removed,
                        old_value: Some(v1.clone()),
                        new_value: None,
                    });
                }
                (None, Some(v2)) => {
                    diffs.push(Diff {
                        path: new_path,
                        diff_type: DiffType::Added,
                        old_value: None,
                        new_value: Some(v2.clone()),
                    });
                }
                (None, None) => {}
            }
        }
    }

    fn values_equal(&self, v1: &Value, v2: &Value) -> bool {
        match (v1, v2) {
            (Value::Null, Value::Null) => true,
            (Value::Bool(b1), Value::Bool(b2)) => b1 == b2,
            (Value::Number(n1), Value::Number(n2)) => n1 == n2,
            (Value::String(s1), Value::String(s2)) => s1 == s2,
            (Value::Array(a1), Value::Array(a2)) => {
                if a1.len() != a2.len() {
                    return false;
                }
                if self.ignore_order {
                    self.arrays_equal_ignore_order(a1, a2)
                } else {
                    a1.iter()
                        .zip(a2.iter())
                        .all(|(v1, v2)| self.values_equal(v1, v2))
                }
            }
            (Value::Object(o1), Value::Object(o2)) => {
                if o1.len() != o2.len() {
                    return false;
                }
                o1.keys().all(|k| match (o1.get(k), o2.get(k)) {
                    (Some(v1), Some(v2)) => self.values_equal(v1, v2),
                    _ => false,
                })
            }
            _ => false,
        }
    }

    fn arrays_equal_ignore_order(&self, a1: &[Value], a2: &[Value]) -> bool {
        if a1.len() != a2.len() {
            return false;
        }

        let mut used = vec![false; a2.len()];

        for v1 in a1 {
            let mut found = false;
            for (j, v2) in a2.iter().enumerate() {
                if !used[j] && self.values_equal(v1, v2) {
                    used[j] = true;
                    found = true;
                    break;
                }
            }
            if !found {
                return false;
            }
        }

        true
    }
}

impl Default for JsonDiff {
    fn default() -> Self {
        Self::new()
    }
}
