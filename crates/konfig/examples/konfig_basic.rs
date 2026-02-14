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
    #[serde(alias = "host_name")]
    host: String,

    #[serde(alias = "server_port")]
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
    let config: AppConfig = Config::new().optional_file("examples/config.toml").load()?;

    println!("Configuration loaded:");
    println!("  Server: {}:{}", config.server.host, config.server.port);
    println!("  Database URL: {}", config.database.url);
    println!("  Max Connections: {}", config.database.max_connections);

    Ok(())
}
