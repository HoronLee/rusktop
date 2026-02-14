#[cfg(feature = "toml")]
use crate::error::Result;
#[cfg(feature = "toml")]
use serde_json::Value;

#[cfg(feature = "toml")]
pub fn parse_toml(content: &str) -> Result<Value> {
    let toml_value: toml::Value = toml::from_str(content)?;

    let json_string = serde_json::to_string(&toml_value)?;
    let json_value: Value = serde_json::from_str(&json_string)?;

    Ok(json_value)
}

#[cfg(not(feature = "toml"))]
pub fn parse_toml(_content: &str) -> Result<Value> {
    use crate::error::ConfigError;
    Err(ConfigError::ParseError(
        "TOML support is not enabled. Enable the 'toml' feature.".to_string(),
    ))
}
