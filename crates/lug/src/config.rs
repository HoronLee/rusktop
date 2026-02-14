//! Configuration types for the lug logging system.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Main configuration for the lug logger.
///
/// # Examples
///
/// ```
/// use lug::{LugConfig, Environment, Level, FileConfig};
/// use std::path::PathBuf;
///
/// let config = LugConfig {
///     env: Environment::Prod,
///     level: Level::Info,
///     file: Some(FileConfig {
///         path: PathBuf::from("/var/log/myapp/app.log"),
///         ..Default::default()
///     }),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LugConfig {
    /// Environment mode (dev, prod, test)
    pub env: Environment,

    /// Minimum log level to output
    pub level: Level,

    /// Optional file logging configuration
    pub file: Option<FileConfig>,
}

/// Environment mode for logging behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    /// Development: Colored terminal output, human-readable format
    Dev,
    /// Production: Console (non-colored) + JSON file output
    Prod,
    /// Test: Silent mode (no output)
    Test,
}

/// Log level following the standard severity hierarchy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    /// Trace: Very fine-grained information (most verbose)
    Trace,
    /// Debug: Debugging information
    Debug,
    /// Info: General informational messages
    Info,
    /// Warn: Warning messages
    Warn,
    /// Error: Error messages
    Error,
}

impl From<Level> for tracing::Level {
    fn from(level: Level) -> Self {
        match level {
            Level::Trace => tracing::Level::TRACE,
            Level::Debug => tracing::Level::DEBUG,
            Level::Info => tracing::Level::INFO,
            Level::Warn => tracing::Level::WARN,
            Level::Error => tracing::Level::ERROR,
        }
    }
}

/// File logging configuration with rotation support.
///
/// # Examples
///
/// ```
/// use lug::FileConfig;
/// use std::path::PathBuf;
///
/// let config = FileConfig {
///     path: PathBuf::from("logs/app.log"),
///     max_size_mb: 50,
///     max_backups: 10,
///     max_age_days: 7,
///     compress: false,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileConfig {
    /// Log file path
    pub path: PathBuf,

    /// Maximum size of a single log file in megabytes
    #[serde(default = "default_max_size")]
    pub max_size_mb: u64,

    /// Maximum number of backup files to retain
    #[serde(default = "default_max_backups")]
    pub max_backups: usize,

    /// Maximum age of log files in days
    #[serde(default = "default_max_age")]
    pub max_age_days: u64,

    /// Whether to compress old log files (currently not implemented)
    #[serde(default)]
    pub compress: bool,
}

fn default_max_size() -> u64 {
    10
}

fn default_max_backups() -> usize {
    5
}

fn default_max_age() -> u64 {
    30
}

impl Default for LugConfig {
    fn default() -> Self {
        Self {
            env: Environment::Prod,
            level: Level::Info,
            file: None,
        }
    }
}

impl Default for FileConfig {
    fn default() -> Self {
        Self {
            path: PathBuf::from("logs/app.log"),
            max_size_mb: 10,
            max_backups: 5,
            max_age_days: 30,
            compress: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_config_defaults() {
        let config = FileConfig::default();
        assert_eq!(config.path, PathBuf::from("logs/app.log"));
        assert_eq!(config.max_size_mb, 10);
        assert_eq!(config.max_backups, 5);
        assert_eq!(config.max_age_days, 30);
        assert!(!config.compress);
    }

    #[test]
    fn test_environment_serialization() {
        let json = serde_json::to_string(&Environment::Dev).unwrap();
        assert_eq!(json, "\"dev\"");

        let env: Environment = serde_json::from_str("\"prod\"").unwrap();
        assert_eq!(env, Environment::Prod);
    }

    #[test]
    fn test_level_ordering() {
        assert!(Level::Error > Level::Warn);
        assert!(Level::Warn > Level::Info);
        assert!(Level::Info > Level::Debug);
        assert!(Level::Debug > Level::Trace);
    }
}
