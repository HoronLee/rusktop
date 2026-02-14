# AGENTS.md

## Project Overview

**Lug** is an opinionated Rust logging wrapper built on `tracing`, inspired by Go's Zap and Kratos frameworks. It provides a batteries-included approach to structured logging with multi-environment support and flexible module tagging.

**Key Features**:
- Multi-environment modes (Dev/Prod/Test)
- Three flexible module tagging patterns
- File rotation with JSON output
- SeaORM integration (optional)
- Rust 2024 Edition

## Build Commands

```bash
# Build the crate
cargo build -p lug

# Build with all features
cargo build -p lug --all-features

# Build for release
cargo build -p lug --release
```

## Testing Instructions

```bash
# Run all tests
cargo test -p lug

# Run only unit tests
cargo test -p lug --lib

# Run integration tests
cargo test -p lug --test integration

# Run doc tests
cargo test -p lug --doc

# Run with verbose output
cargo test -p lug -- --nocapture
```

**Expected Result**: All 28 tests must pass (8 unit + 6 integration + 14 doc tests).

## Examples

Run examples to verify functionality:

```bash
# Basic usage (colored terminal output)
cargo run -p lug --example lug_basic

# Module tagging patterns demonstration
cargo run -p lug --example module_modes

# Production configuration (creates JSON log file)
cargo run -p lug --example production

# SeaORM integration (requires feature flag)
cargo run -p lug --example sea_orm --features sea-orm-integration
```

## Code Style Guidelines

### Rust Edition and Version
- **Edition**: 2024
- **Minimum Rust Version**: 1.85.0 (required by Edition 2024)
- Use `cargo fmt` before committing
- Use `cargo clippy` to catch common mistakes

### Documentation
- All public items must have doc comments (`///`)
- Include examples in doc comments where applicable
- Doc tests should use `no_run` for examples that require initialization

### Error Handling
- Use `thiserror` for error types
- All errors must implement `std::error::Error`
- Error messages should be descriptive and actionable

### Module Organization
```
src/
├── lib.rs          # Public API + macros
├── config.rs       # Configuration types (serializable)
├── error.rs        # Error types
├── layer.rs        # File logging implementation
└── integrations/   # Optional integrations
    ├── mod.rs
    └── sea_orm.rs
```

## Dependencies

### Core Dependencies (always required)
- `tracing` 0.1.44 - Core tracing framework
- `tracing-subscriber` 0.3.22 - Subscriber implementations
- `tracing-appender` 0.2.4 - File rotation
- `serde` 1.0 - Serialization
- `thiserror` 2.0 - Error handling
- `chrono` 0.4 - Timestamp formatting

### Optional Dependencies
- `sea-orm` 1.1 - ORM integration (feature: `sea-orm-integration`)

### Dev Dependencies
- `tokio` 1.x - Async runtime (for examples)
- `tempfile` 3.x - Temporary directories (for tests)
- `serde_json` 1.0 - JSON serialization (for tests)

## Feature Flags

```toml
[features]
default = []
sea-orm-integration = ["sea-orm"]
```

Use `--features sea-orm-integration` to enable SeaORM support.

## Commit Guidelines

- Commits must pass `cargo test -p lug`
- Commits must pass `cargo clippy -p lug`
- Run `cargo fmt` before committing
- Follow conventional commit format:
  - `feat(lug): add new feature`
  - `fix(lug): fix bug description`
  - `docs(lug): update documentation`
  - `test(lug): add missing tests`

## Common Patterns

### Module Tagging Convention
Format: `[component]/[layer]/[service]`

Examples:
- `auth/biz/myapp` - Authentication business logic
- `redis/data/myapp` - Redis data access
- `http/api/myapp` - HTTP API layer

### Three Module Tagging Modes

1. **Span Mode** (Recommended):
   ```rust
   let _guard = module_span!("auth/biz/myapp").entered();
   ```

2. **Scoped Macro**:
   ```rust
   with_module!("redis/data/myapp", { /* code */ });
   ```

3. **Inline Macro**:
   ```rust
   lug::info!(module: "api/http/myapp", "message");
   ```

## Security Considerations

- Never log sensitive data (passwords, tokens, API keys) in plain text
- Use structured fields instead of string interpolation
- Sanitize user input before logging
- Be cautious with `Debug` formatting on external types

## Performance Notes

- Disabled log levels have **zero runtime cost** (thanks to `tracing`)
- Prefer structured fields over format strings
- File I/O uses buffered writers
- JSON serialization is deferred for better performance

## Known Limitations

1. **File Rotation**: Currently only supports time-based (daily) rotation. Size-based rotation is planned but not implemented.
2. **Compression**: `compress` field in `FileConfig` is not yet functional.
3. **Slow Query Detection**: SeaORM slow query warnings require custom implementation.

## Future Enhancements

See `IMPLEMENTATION.md` for roadmap:
- Size-based file rotation
- Log compression (gzip)
- OpenTelemetry integration
- Syslog/journald output

## Troubleshooting

### Tests failing?
- Ensure Rust >= 1.85.0: `rustc --version`
- Update dependencies: `cargo update`
- Clean build: `cargo clean && cargo build -p lug`

### LSP errors in IDE?
- Run `cargo check -p lug` to ensure compilation
- Restart your language server
- Verify `rust-analyzer` is up to date

### Example not running?
- Check if feature flags are needed (e.g., `sea_orm` example)
- Verify you're in the workspace root
- Use `-p lug` to specify the package

## Additional Resources

- **README.md**: Quick start and overview
- **GUIDE.md**: Detailed usage guide with examples
- **IMPLEMENTATION.md**: Full implementation details and roadmap
- **API Docs**: Run `cargo doc --open -p lug`
