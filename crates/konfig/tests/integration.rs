use konfig::Config;
use serde::{Deserialize, Serialize};
use std::io::Write;
use tempfile::NamedTempFile;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct TestConfig {
    #[serde(default = "default_name")]
    name: String,
    #[serde(default)]
    value: i32,
}

fn default_name() -> String {
    "default".to_string()
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            name: default_name(),
            value: 0,
        }
    }
}

#[test]
fn test_load_from_toml() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(
        temp_file,
        r#"
name = "test"
value = 42
    "#
    )
    .unwrap();

    let config: TestConfig = Config::new().file(temp_file.path()).load().unwrap();

    assert_eq!(config.name, "test");
    assert_eq!(config.value, 42);
}

#[test]
fn test_default_values() {
    let config: TestConfig = Config::new()
        .optional_file("nonexistent.toml")
        .load()
        .unwrap();

    assert_eq!(config.name, "default");
    assert_eq!(config.value, 0);
}

#[test]
fn test_env_override() {
    unsafe {
        std::env::set_var("TEST_NAME", "from_env");
        std::env::set_var("TEST_VALUE", "99");
    }

    let config: TestConfig = Config::new().env_prefix("TEST").load().unwrap();

    assert_eq!(config.name, "from_env");
    assert_eq!(config.value, 99);

    unsafe {
        std::env::remove_var("TEST_NAME");
        std::env::remove_var("TEST_VALUE");
    }
}

#[test]
fn test_priority_env_over_file() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(
        temp_file,
        r#"
name = "from_file"
value = 10
    "#
    )
    .unwrap();

    unsafe {
        std::env::set_var("PRIORITY_VALUE", "20");
    }

    let config: TestConfig = Config::new()
        .env_prefix("PRIORITY")
        .file(temp_file.path())
        .load()
        .unwrap();

    assert_eq!(config.name, "from_file");
    assert_eq!(config.value, 20);

    unsafe {
        std::env::remove_var("PRIORITY_VALUE");
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(default)]
struct NestedConfig {
    server: ServerConfig,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
struct ServerConfig {
    host: String,
    port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 8080,
        }
    }
}

#[test]
fn test_nested_env_override() {
    unsafe {
        std::env::set_var("NESTED_SERVER_PORT", "9000");
    }

    let config: NestedConfig = Config::new().env_prefix("NESTED").load().unwrap();

    assert_eq!(config.server.host, "localhost");
    assert_eq!(config.server.port, 9000);

    unsafe {
        std::env::remove_var("NESTED_SERVER_PORT");
    }
}
