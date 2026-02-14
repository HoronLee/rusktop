use crate::value::ConfigMap;
use serde_json::Value;

pub fn merge_deep(base: &mut ConfigMap, overlay: ConfigMap) {
    for (key, value) in overlay {
        base.entry(key)
            .and_modify(|base_val| deep_merge_value(base_val, value.clone()))
            .or_insert(value);
    }
}

fn deep_merge_value(base: &mut Value, overlay: Value) {
    match (base, overlay) {
        (Value::Object(base_map), Value::Object(overlay_map)) => {
            for (key, value) in overlay_map {
                base_map
                    .entry(key)
                    .and_modify(|base_val| deep_merge_value(base_val, value.clone()))
                    .or_insert(value);
            }
        }
        (base_val, overlay_val) => {
            *base_val = overlay_val;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_merge_simple() {
        let mut base = ConfigMap::new();
        base.insert("a".to_string(), json!(1));

        let mut overlay = ConfigMap::new();
        overlay.insert("b".to_string(), json!(2));

        merge_deep(&mut base, overlay);

        assert_eq!(base.get("a"), Some(&json!(1)));
        assert_eq!(base.get("b"), Some(&json!(2)));
    }

    #[test]
    fn test_merge_override() {
        let mut base = ConfigMap::new();
        base.insert("a".to_string(), json!(1));

        let mut overlay = ConfigMap::new();
        overlay.insert("a".to_string(), json!(2));

        merge_deep(&mut base, overlay);

        assert_eq!(base.get("a"), Some(&json!(2)));
    }

    #[test]
    fn test_merge_nested() {
        let mut base = ConfigMap::new();
        base.insert("server".to_string(), json!({"host": "localhost"}));

        let mut overlay = ConfigMap::new();
        overlay.insert("server".to_string(), json!({"port": 8080}));

        merge_deep(&mut base, overlay);

        assert_eq!(
            base.get("server"),
            Some(&json!({"host": "localhost", "port": 8080}))
        );
    }
}
