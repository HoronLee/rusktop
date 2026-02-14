# AGENTS.md

## Project Overview

**konfig** is a flexible configuration management library for Rust, inspired by Go's Viper.

- **Type**: Library crate
- **Language**: Rust 2024 edition
- **Purpose**: Load and merge configuration from multiple sources (files, environment variables, defaults)
- **Design Philosophy**: Simplicity over features - uses serde for type safety, no complex migration system

## Key Features

- Multi-source configuration: TOML/JSON/YAML files + environment variables + defaults
- Priority-based merging: `env > file > default`
- Type-safe deserialization via serde
- Environment variable auto-mapping: `APP_SERVER_PORT` → `server.port`
- Zero runtime overhead for config evolution (handled at compile-time via serde)

## Setup Commands

```bash
# Build the library
cargo build

# Run all tests
cargo test

# Run tests with all features enabled
cargo test --all-features

# Build release version
cargo build --release

# Run examples
cargo run --example basic
TEST_PORT=9000 cargo run --example debug
APP_SERVER_PORT=9000 cargo run --example env_override
```

## Project Structure

```
crates/konfig/
├── src/
│   ├── lib.rs              # Public API exports
│   ├── error.rs            # Error types using thiserror
│   ├── config.rs           # Config entry point
│   ├── builder.rs          # ConfigBuilder (main user-facing API)
│   ├── value.rs            # ConfigMap utilities, nested value access
│   ├── merge.rs            # Deep merge strategy for configs
│   ├── source/             # Configuration sources
│   │   ├── traits.rs       # ConfigSource trait
│   │   ├── file.rs         # FileSource (TOML/JSON/YAML)
│   │   ├── env.rs          # EnvSource (environment variables)
│   │   └── default.rs      # DefaultSource (from Default trait)
│   └── format/             # Format parsers
│       ├── toml.rs         # TOML parser (feature-gated)
│       ├── yaml.rs         # YAML parser (feature-gated)
│       └── json.rs         # JSON (always enabled)
├── examples/               # Usage examples
├── tests/                  # Integration tests
└── Cargo.toml
```

## Code Style

- **Rust edition**: 2024
- **MSRV**: 1.85
- **Error handling**: Use `thiserror` for error types, return `Result<T>`
- **Naming**: Snake_case for functions/variables, PascalCase for types
- **Imports**: Group std → external crates → local crates
- **Documentation**: Public items should have doc comments
- **Unsafe**: Avoided - previous unsafe code in `value.rs` was refactored to safe recursion

## Architecture Principles

### 1. Simplicity Over Features
- **NO Migration API**: Config evolution is handled via serde attributes (`#[serde(default)]`, `#[serde(alias)]`)
- Old config files remain valid, unknown fields are ignored
- New fields get default values automatically

### 2. Type Safety
- All config access is type-safe via serde's `DeserializeOwned`
- ConfigBuilder requires `T: DeserializeOwned + Default + Serialize`
- Internal representation uses `serde_json::Value` for flexibility

### 3. Priority Merging
- Sources are merged in order: default → file → env
- Deep merge preserves nested structures
- Last source wins for leaf values

### 4. Environment Variable Mapping
- Prefix-based filtering: `APP_` prefix → only vars starting with `APP_`
- Separator conversion: `_` → `.` (e.g., `APP_SERVER_PORT` → `server.port`)
- Case normalization: uppercase env vars → lowercase config keys
- Type inference: attempts to parse as number/bool, falls back to string

## Testing Instructions

### Unit Tests
```bash
# Merge strategy tests
cargo test merge::tests

# Run specific test
cargo test test_nested_env_override
```

### Integration Tests
Located in `tests/integration.rs`:
- `test_load_from_toml`: File loading
- `test_default_values`: Default trait integration
- `test_env_override`: Environment variable override
- `test_priority_env_over_file`: Priority merging
- `test_nested_env_override`: Nested environment vars

### Examples as Tests
Run examples to verify functionality:
```bash
cargo run --example basic           # Should load from config.toml
cargo run --example env_override    # Should show env var override
TEST_PORT=9000 cargo run --example debug  # Should output port=9000
```

## Common Development Tasks

### Adding a New Format
1. Add optional dependency in `Cargo.toml`
2. Create `src/format/{format}.rs` with `parse_{format}(content: &str) -> Result<Value>`
3. Feature-gate with `#[cfg(feature = "format")]`
4. Add to `FileFormat` enum in `src/source/file.rs`
5. Update README with new format support

### Adding a New ConfigSource
1. Implement `ConfigSource` trait in `src/source/{name}.rs`
2. Add `fn load(&self) -> Result<ConfigMap>` method
3. Export from `src/source/mod.rs`
4. Add builder method in `ConfigBuilder`
5. Write tests in `tests/integration.rs`

### Debugging Config Loading
Enable debug example to inspect environment variables:
```bash
RUST_LOG=debug TEST_SOME_VAR=value cargo run --example debug
```

## Dependencies

### Core (always included)
- `serde = "1.0.228"` - Serialization framework
- `serde_json = "1.0.149"` - JSON support + internal representation
- `thiserror = "2.0.18"` - Error derive macros

### Optional (feature-gated)
- `toml = "1.0.1"` - TOML support (enabled by default)
- `serde_yaml_ng = "0.10"` - YAML support (maintained fork, not `serde_yaml`)

### Dev Dependencies
- `tempfile = "3.0"` - Temporary files for integration tests

## Feature Flags

```toml
default = ["toml"]
toml = ["dep:toml"]
yaml = ["dep:serde_yaml_ng"]
all-formats = ["toml", "yaml"]
```

## Security Considerations

- No `unsafe` code (removed from `value.rs` in favor of safe recursion)
- Environment variables are parsed carefully (type inference with fallback)
- File paths are validated before reading
- No arbitrary code execution from config files

## Common Pitfalls

### 1. serde(alias) + JSON intermediate representation
- `#[serde(alias = "old_name")]` may not work as expected when config goes through JSON
- Reason: TOML → JSON → Struct conversion can create duplicate fields
- Solution: Document that users should migrate old field names in their config files

### 2. ConfigBuilder requires Serialize
- `T: DeserializeOwned + Default + Serialize` is needed
- Reason: We serialize `T::default()` to get default values as JSON
- Solution: Always derive both `Deserialize` and `Serialize` on config structs

### 3. Nested environment variables
- `APP_SERVER_PORT` works, `APP_SERVER_CONFIG_PORT` creates `server.config.port`
- Each `_` becomes a `.` in the key path
- Solution: Document env var naming clearly in project docs

## PR Guidelines

- Run `cargo test --all-features` before committing
- Run `cargo clippy` and fix warnings
- Update README.md if adding new features
- Add integration test for new ConfigSource types
- Keep examples up-to-date with API changes

## Useful Commands

```bash
# Check for unused dependencies
cargo +nightly udeps

# Format code
cargo fmt

# Lint code
cargo clippy -- -D warnings

# Check documentation
cargo doc --no-deps --open

# Benchmark (if added in future)
cargo bench
```

## Known Limitations

- serde `#[serde(alias)]` not fully supported due to JSON intermediate representation
- No watch/reload functionality for config files (could be added as feature)
- No validation framework (users should implement validation in their Default impl or separately)
- No config writing/saving (read-only by design)
