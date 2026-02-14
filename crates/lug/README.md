# Lug

[![Crates.io](https://img.shields.io/crates/v/lug.svg)](https://crates.io/crates/lug)
[![Documentation](https://docs.rs/lug/badge.svg)](https://docs.rs/lug)
[![License](https://img.shields.io/crates/l/lug.svg)](https://github.com/yourusername/rusktop/tree/main/crates/lug)

**Lug** is an opinionated Rust logging wrapper built on [`tracing`](https://github.com/tokio-rs/tracing), inspired by Go's [Zap](https://github.com/uber-go/zap) and [Kratos](https://github.com/go-kratos/kratos) frameworks.

## Features

- 🎨 **Multi-environment support**: Dev (colored console), Prod (console + JSON file), Test (silent)
- 📁 **File rotation**: Daily rotation with configurable retention
- 🏷️ **Flexible module tagging**: Three patterns for adding context to logs
- 🔌 **SeaORM integration**: Optional ORM logging support
- ⚡ **Built on `tracing`**: Leverages the modern Rust observability ecosystem
- 🦀 **Rust 2024 Edition**: Uses the latest stable Rust features

## Quick Start

Add `lug` to your `Cargo.toml`:

```toml
[dependencies]
lug = "0.1"
```

Initialize the logger:

```rust
use lug::{LugConfig, Environment, Level};

fn main() {
    lug::init(LugConfig {
        env: Environment::Dev,
        level: Level::Debug,
        file: None,
    }).expect("Failed to initialize logger");

    tracing::info!("Hello, world!");
}
```

## Module Tagging Patterns

Lug provides three flexible patterns for adding module context to your logs:

### Pattern 1: Span Mode (Recommended)

Automatically propagates context across async boundaries and nested function calls:

```rust
use lug::module_span;

let _guard = module_span!("auth/biz/myapp").entered();
tracing::info!(user_id = 42, "Login attempt");
// Output: {"module":"auth/biz/myapp","user_id":42,"message":"Login attempt"}
```

### Pattern 2: Scoped Macro

Execute code within a module-tagged scope:

```rust
use lug::with_module;

with_module!("redis/data/myapp", {
    tracing::info!("Connected to Redis");
});
```

### Pattern 3: Inline Macro

Add module tags to individual log statements:

```rust
lug::info!(module: "metrics/api/myapp", counter = 100, "Metric recorded");
lug::warn!(module: "cache/data/myapp", "Cache miss");
```

**Module naming convention**: `[component]/[layer]/[service]`
- Examples: `"redis/data/myapp"`, `"auth/biz/myapp"`, `"http/api/myapp"`

## Configuration

### Development Environment

```rust
use lug::{LugConfig, Environment, Level};

lug::init(LugConfig {
    env: Environment::Dev,
    level: Level::Debug,
    file: None,
}).unwrap();
```

**Output**: Colored terminal logs with timestamps

### Production Environment

```rust
use lug::{LugConfig, Environment, Level, FileConfig};
use std::path::PathBuf;

lug::init(LugConfig {
    env: Environment::Prod,
    level: Level::Info,
    file: Some(FileConfig {
        path: PathBuf::from("/var/log/myapp/app.log"),
        max_size_mb: 50,
        max_backups: 10,
        max_age_days: 30,
        compress: false,
    }),
}).unwrap();
```

**Output**: 
- Console: Non-colored structured logs
- File: JSON format with daily rotation

### Test Environment

```rust
lug::init(LugConfig {
    env: Environment::Test,
    ..Default::default()
}).unwrap();
```

**Output**: Silent (no logs)

## SeaORM Integration

Enable the `sea-orm-integration` feature:

```toml
[dependencies]
lug = { version = "0.1", features = ["sea-orm-integration"] }
sea-orm = "1.1"
```

Configure SeaORM logging:

```rust
use lug::integrations::configure_sea_orm;
use sea_orm::{Database, ConnectOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    lug::init(lug::LugConfig::default())?;

    let mut opt = ConnectOptions::new("postgres://localhost/mydb");
    configure_sea_orm(&mut opt, "database/data/myapp");

    let db = Database::connect(opt).await?;
    // SQL queries are now logged with module="database/data/myapp"

    Ok(())
}
```

## Environment Variable Control

Lug respects the `RUST_LOG` environment variable:

```bash
# Set global log level
RUST_LOG=debug cargo run

# Filter by module
RUST_LOG=myapp::auth=debug cargo run

# See SQL queries (with SeaORM)
RUST_LOG=sqlx=debug cargo run
```

## Examples

Run the examples to see lug in action:

```bash
# Basic usage
cargo run --example lug_basic

# Module tagging patterns
cargo run --example module_modes

# Production configuration
cargo run --example production

# SeaORM integration (requires feature)
cargo run --example sea_orm_example --features sea-orm-integration
```

## Comparison with Go Version

| Feature | Go (Zap/Kratos) | Rust (lug) |
|---------|----------------|-----------|
| **Structured Logging** | `zap.String("key", val)` | `tracing::info!(key = val)` |
| **Environment Modes** | `switch c.Env` | `match config.env` |
| **File Rotation** | Lumberjack (size-based) | `tracing-appender` (time-based) |
| **Module Tags** | `WithModule()` (returns new logger) | 3 flexible patterns (span/macro/inline) |
| **Async Support** | Limited | Native (via `tracing`) |
| **ORM Integration** | GORM adapter | SeaORM integration |

## Roadmap

- [ ] Size-based file rotation (currently only daily rotation)
- [ ] Log compression support (gzip)
- [ ] Slow query detection for SeaORM
- [ ] OpenTelemetry integration
- [ ] Syslog/journald output

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Inspiration

Lug is inspired by:
- [uber-go/zap](https://github.com/uber-go/zap) - High-performance Go logging
- [go-kratos/kratos](https://github.com/go-kratos/kratos) - Go microservice framework
- [tokio-rs/tracing](https://github.com/tokio-rs/tracing) - Rust application-level tracing
