#[cfg(feature = "yaml")]
use crate::error::{ConfigError, Result};
#[cfg(feature = "yaml")]
use serde_json::Value;

#[cfg(feature = "yaml")]
pub fn parse_yaml(content: &str) -> Result<Value> {
    let yaml_value: serde_yaml_ng::Value =
        serde_yaml_ng::from_str(content).map_err(|e| ConfigError::YamlError(e.to_string()))?;

    let json_string = serde_json::to_string(&yaml_value)?;
    let json_value: Value = serde_json::from_str(&json_string)?;

    Ok(json_value)
}

#[cfg(not(feature = "yaml"))]
pub fn parse_yaml(_content: &str) -> Result<Value> {
    Err(ConfigError::ParseError(
        "YAML support is not enabled. Enable the 'yaml' feature.".to_string(),
    ))
}
