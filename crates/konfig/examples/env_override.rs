use konfig::Config;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(default)]
struct AppConfig {
    server: ServerConfig,
    database: DatabaseConfig,
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
            host: "0.0.0.0".to_string(),
            port: 8080,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
struct DatabaseConfig {
    url: String,
    max_connections: u32,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "sqlite::memory:".to_string(),
            max_connections: 10,
        }
    }
}

fn main() -> konfig::Result<()> {
    println!("Example: Environment Variable Override");
    println!("Set environment variables like:");
    println!("  APP_SERVER_PORT=9000");
    println!("  APP_DATABASE_MAX_CONNECTIONS=50");
    println!();

    let config: AppConfig = Config::new()
        .env_prefix("APP")
        .optional_file("examples/config.toml")
        .load()?;

    println!("Configuration loaded:");
    println!("  Server: {}:{}", config.server.host, config.server.port);
    println!("  Database URL: {}", config.database.url);
    println!("  Max Connections: {}", config.database.max_connections);

    Ok(())
}
