//! Integration tests for the lug logger.

use lug::{Environment, FileConfig, Level, LugConfig};
use tempfile::TempDir;

#[test]
fn test_dev_environment_init() {
    let config = LugConfig {
        env: Environment::Dev,
        level: Level::Debug,
        file: None,
    };

    // Note: We can't actually init twice in tests, so we just verify config creation
    assert_eq!(config.env, Environment::Dev);
    assert_eq!(config.level, Level::Debug);
}

#[test]
fn test_file_config_creation() {
    let temp_dir = TempDir::new().unwrap();
    let log_path = temp_dir.path().join("test.log");

    let file_config = FileConfig {
        path: log_path.clone(),
        max_size_mb: 20,
        max_backups: 3,
        max_age_days: 7,
        compress: true,
    };

    assert_eq!(file_config.path, log_path);
    assert_eq!(file_config.max_size_mb, 20);
    assert_eq!(file_config.max_backups, 3);
    assert_eq!(file_config.max_age_days, 7);
    assert!(file_config.compress);
}

#[test]
fn test_prod_environment_with_file() {
    let temp_dir = TempDir::new().unwrap();
    let log_path = temp_dir.path().join("app.log");

    let config = LugConfig {
        env: Environment::Prod,
        level: Level::Info,
        file: Some(FileConfig {
            path: log_path.clone(),
            ..Default::default()
        }),
    };

    assert_eq!(config.env, Environment::Prod);
    assert!(config.file.is_some());
}

#[test]
fn test_serialization() {
    let config = LugConfig {
        env: Environment::Dev,
        level: Level::Debug,
        file: Some(FileConfig::default()),
    };

    // Test JSON serialization
    let json = serde_json::to_string(&config).unwrap();
    let deserialized: LugConfig = serde_json::from_str(&json).unwrap();

    assert_eq!(config.env, deserialized.env);
    assert_eq!(config.level, deserialized.level);
}

#[test]
fn test_module_span_macro() {
    // Just ensure the macro compiles and expands correctly
    let _span = lug::module_span!("test/module");
}

#[test]
fn test_inline_log_macros() {
    // Ensure all log macros compile
    // Note: These won't actually log without init
    let _info = || lug::info!(module: "test", "test message");
    let _warn = || lug::warn!(module: "test", "test warning");
    let _error = || lug::error!(module: "test", "test error");
    let _debug = || lug::debug!(module: "test", "test debug");
    let _trace = || lug::trace!(module: "test", "test trace");
}
