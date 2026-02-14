use crate::error::Result;
use crate::source::traits::ConfigSource;
use crate::value::{set_nested_value, ConfigMap};
use serde_json::Value;

pub struct EnvSource {
    prefix: Option<String>,
    separator: String,
}

impl EnvSource {
    pub fn new(prefix: Option<String>) -> Self {
        Self {
            prefix,
            separator: "_".to_string(),
        }
    }

    pub fn with_separator(mut self, separator: String) -> Self {
        self.separator = separator;
        self
    }

    fn parse_env_value(value: &str) -> Value {
        if let Ok(num) = value.parse::<i64>() {
            return Value::Number(num.into());
        }

        if let Ok(num) = value.parse::<f64>() {
            if let Some(n) = serde_json::Number::from_f64(num) {
                return Value::Number(n);
            }
        }

        if let Ok(b) = value.parse::<bool>() {
            return Value::Bool(b);
        }

        Value::String(value.to_string())
    }
}

impl ConfigSource for EnvSource {
    fn load(&self) -> Result<ConfigMap> {
        let mut config_map = ConfigMap::new();

        for (key, value) in std::env::vars() {
            let config_key = if let Some(prefix) = &self.prefix {
                let prefix_with_sep = format!("{}{}", prefix, self.separator);
                if let Some(stripped) = key.strip_prefix(&prefix_with_sep) {
                    stripped.to_lowercase().replace(&self.separator, ".")
                } else {
                    continue;
                }
            } else {
                key.to_lowercase().replace(&self.separator, ".")
            };

            let json_value = Self::parse_env_value(&value);
            set_nested_value(&mut config_map, &config_key, json_value);
        }

        Ok(config_map)
    }

    fn name(&self) -> &str {
        "environment"
    }
}
