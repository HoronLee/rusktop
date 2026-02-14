use serde_json::Value;
use std::collections::HashMap;

pub type ConfigValue = Value;
pub type ConfigMap = HashMap<String, Value>;

pub fn get_nested_value<'a>(map: &'a ConfigMap, key: &str) -> Option<&'a Value> {
    let parts: Vec<&str> = key.split('.').collect();
    let mut current = None;

    for (i, part) in parts.iter().enumerate() {
        if i == 0 {
            current = map.get(*part);
        } else {
            current = current.and_then(|v| v.get(part));
        }

        if current.is_none() {
            return None;
        }
    }

    current
}

pub fn set_nested_value(map: &mut ConfigMap, key: &str, value: Value) {
    let parts: Vec<&str> = key.split('.').collect();

    if parts.len() == 1 {
        map.insert(key.to_string(), value);
        return;
    }

    fn insert_at_path(
        current_map: &mut serde_json::Map<String, Value>,
        parts: &[&str],
        value: Value,
    ) {
        if parts.len() == 1 {
            current_map.insert(parts[0].to_string(), value);
            return;
        }

        let key = parts[0];
        let entry = current_map
            .entry(key.to_string())
            .or_insert_with(|| Value::Object(serde_json::Map::new()));

        if let Value::Object(next_map) = entry {
            insert_at_path(next_map, &parts[1..], value);
        }
    }

    let root_value = map
        .entry(parts[0].to_string())
        .or_insert_with(|| Value::Object(serde_json::Map::new()));

    if let Value::Object(root_map) = root_value {
        insert_at_path(root_map, &parts[1..], value);
    }
}
