use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    FileRead(#[from] std::io::Error),

    #[error("Failed to parse config: {0}")]
    ParseError(String),

    #[error("Key not found: {0}")]
    KeyNotFound(String),

    #[error("Type mismatch for key '{key}': expected {expected}")]
    TypeMismatch { key: String, expected: String },

    #[error("Environment variable error: {0}")]
    EnvError(String),

    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),

    #[cfg(feature = "toml")]
    #[error("TOML parse error: {0}")]
    TomlError(#[from] toml::de::Error),

    #[cfg(feature = "yaml")]
    #[error("YAML parse error: {0}")]
    YamlError(String),
}

pub type Result<T> = std::result::Result<T, ConfigError>;
