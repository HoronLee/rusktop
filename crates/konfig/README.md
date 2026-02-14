# Konfig

A flexible configuration management library for Rust, inspired by Go's Viper.

## Features

- ✅ **Multiple configuration sources**: Files (TOML, JSON, YAML*), environment variables, and default values
- ✅ **Priority-based merging**: Environment variables > Files > Defaults
- ✅ **Type-safe**: Leverages serde for deserialization into your structs
- ✅ **Simple API**: Clean, intuitive builder pattern
- ✅ **Zero runtime overhead**: Configuration evolution handled at compile-time using serde
- ✅ **No migrations needed**: Use serde attributes (`#[serde(default)]`, `#[serde(alias)]`) for backward compatibility

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
konfig = "0.1"
```

Or with specific format support:

```toml
[dependencies]
konfig = { version = "0.1", features = ["toml", "yaml"] }
```

## Quick Start

### Define your configuration structure

```rust
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
```

### Load configuration

```rust
fn main() -> konfig::Result<()> {
    let config: AppConfig = Config::new()
        .env_prefix("APP")              // Environment variables with APP_ prefix
        .file("config.toml")            // Load from file
        .load()?;                       // Load and deserialize
    
    println!("Server: {}:{}", config.server.host, config.server.port);
    Ok(())
}
```

### Configuration file (`config.toml`)

```toml
[server]
host = "localhost"
port = 3000

[database]
url = "postgres://localhost/mydb"
max_connections = 20
```

### Environment variable override

```bash
APP_SERVER_PORT=9000 APP_DATABASE_MAX_CONNECTIONS=50 cargo run
```

**Priority**: `APP_SERVER_PORT=9000` > `port = 3000` in file > `port: 8080` in default

## Configuration Evolution

Konfig handles configuration changes gracefully using serde attributes:

### Renaming fields

```rust
#[derive(Deserialize)]
struct Config {
    // Old config files can still use "server_port"
    #[serde(alias = "server_port")]
    port: u16,
}
```

### Adding new fields

```rust
#[derive(Deserialize)]
struct Config {
    // New field with default value
    #[serde(default = "default_timeout")]
    timeout: u32,
}

fn default_timeout() -> u32 { 30 }
```

### Removing fields

Simply remove the field from your struct - old config files will ignore unknown fields.

## API Reference

### ConfigBuilder

```rust
Config::new()
    .env_prefix("APP")              // Set environment variable prefix
    .env_separator("__")            // Set separator (default: "_")
    .file("config.toml")            // Add config file (fails if not found)
    .optional_file("local.toml")    // Add optional config file
    .load::<AppConfig>()?;          // Load and deserialize
```

### Environment Variable Mapping

- `APP_SERVER_PORT` → `server.port`
- `APP_DATABASE_URL` → `database.url`
- Automatic conversion: `_` → `.`
- Automatic lowercasing

### Supported Formats

| Format | Feature Flag | Extensions |
|--------|--------------|------------|
| JSON   | (always enabled) | `.json` |
| TOML   | `toml` (default) | `.toml` |
| YAML   | `yaml` | `.yaml`, `.yml` |

## Examples

See the [`examples/`](examples/) directory for more:

- [`konfig_basic.rs`](examples/konfig_basic.rs) - Basic configuration loading
- [`env_override.rs`](examples/env_override.rs) - Environment variable override
- [`debug.rs`](examples/debug.rs) - Debug environment variables

Run examples:

```bash
cargo run --example konfig_basic
APP_SERVER_PORT=9000 cargo run --example env_override
```

## Testing

```bash
cargo test
```

## License

MIT OR Apache-2.0
