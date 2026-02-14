//! # Lug - Opinionated Rust Logging Wrapper
//!
//! `lug` is a structured logging wrapper inspired by Go's Zap and Kratos frameworks.
//! It provides a batteries-included approach to logging with multiple environment modes,
//! file rotation, and flexible module tagging.
//!
//! ## Quick Start
//!
//! ```no_run
//! use lug::{LugConfig, Environment, Level};
//!
//! // Initialize the logger
//! lug::init(LugConfig {
//!     env: Environment::Dev,
//!     level: Level::Debug,
//!     file: None,
//! }).expect("Failed to initialize logger");
//!
//! // Use with module tags
//! tracing::info!(module = "auth/biz/myapp", user_id = 42, "User logged in");
//! ```
//!
//! ## Features
//!
//! - **Multi-environment support**: Dev (colored console), Prod (console + JSON file), Test (silent)
//! - **File rotation**: Daily rotation with configurable retention
//! - **Flexible module tagging**: Three different patterns for adding module context
//! - **SeaORM integration**: Optional ORM logging support (feature: `sea-orm-integration`)
//!
//! ## Module Tagging Patterns
//!
//! ### Pattern 1: Span Mode (Recommended)
//! ```no_run
//! # use lug::module_span;
//! let _guard = module_span!("auth/biz/myapp").entered();
//! tracing::info!(user_id = 42, "Login attempt");
//! // All logs in this scope include module="auth/biz/myapp"
//! ```
//!
//! ### Pattern 2: Scoped Macro
//! ```no_run
//! # use lug::with_module;
//! with_module!("redis/data/myapp", {
//!     tracing::info!("Connected to Redis");
//! });
//! ```
//!
//! ### Pattern 3: Inline Macro
//! ```no_run
//! lug::info!(module: "metrics/api/myapp", counter = 100, "Metric recorded");
//! ```

use tracing_subscriber::{prelude::*, EnvFilter, Registry};

mod config;
mod error;
mod layer;

#[cfg(feature = "sea-orm-integration")]
pub mod integrations;

pub use config::{Environment, FileConfig, Level, LugConfig};
pub use error::LugError;

/// Initialize the global logging system.
///
/// This function should be called once at application startup.
/// Subsequent calls will return an error.
///
/// # Examples
///
/// ```no_run
/// use lug::{LugConfig, Environment, Level};
///
/// lug::init(LugConfig {
///     env: Environment::Dev,
///     level: Level::Info,
///     file: None,
/// }).expect("Failed to initialize logger");
/// ```
///
/// # Errors
///
/// Returns `LugError::InitFailed` if:
/// - A global subscriber has already been set
/// - File logging setup fails (e.g., permission denied)
pub fn init(config: LugConfig) -> Result<(), LugError> {
    let filter = build_filter(&config);

    match config.env {
        Environment::Dev => {
            // Dev: Colored terminal output, human-readable format
            Registry::default()
                .with(filter)
                .with(
                    tracing_subscriber::fmt::layer()
                        .with_ansi(true)
                        .with_target(false)
                        .with_thread_ids(true)
                        .with_timer(tracing_subscriber::fmt::time::ChronoLocal::rfc_3339()),
                )
                .try_init()
                .map_err(|e| LugError::InitFailed(e.to_string()))?;
        }

        Environment::Prod => {
            // Prod: Non-colored console + JSON file
            let console_layer = tracing_subscriber::fmt::layer()
                .with_ansi(false)
                .with_target(true)
                .with_timer(tracing_subscriber::fmt::time::ChronoLocal::rfc_3339());

            if let Some(file_cfg) = config.file {
                let file_layer = layer::create_file_layer(&file_cfg)?;
                Registry::default()
                    .with(filter)
                    .with(console_layer)
                    .with(file_layer)
                    .try_init()
                    .map_err(|e| LugError::InitFailed(e.to_string()))?;
            } else {
                Registry::default()
                    .with(filter)
                    .with(console_layer)
                    .try_init()
                    .map_err(|e| LugError::InitFailed(e.to_string()))?;
            }
        }

        Environment::Test => {
            // Test: Completely silent
            Registry::default()
                .with(EnvFilter::new("off"))
                .try_init()
                .map_err(|e| LugError::InitFailed(e.to_string()))?;
        }
    }

    Ok(())
}

fn build_filter(config: &LugConfig) -> EnvFilter {
    // Prioritize RUST_LOG environment variable, fallback to config level
    EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        let level: tracing::Level = config.level.into();
        EnvFilter::new(level.as_str())
    })
}

/// Pattern 1: Create a span with a module tag (Recommended).
///
/// This pattern is recommended because spans automatically propagate context
/// across async boundaries and nested function calls.
///
/// # Examples
///
/// ```no_run
/// use lug::module_span;
///
/// let _guard = module_span!("auth/biz/myapp").entered();
/// tracing::info!(user_id = 42, "Login attempt");
/// // Output includes: module="auth/biz/myapp"
/// ```
///
/// Module naming convention: `[component]/[layer]/[service]`
/// - Examples: `"redis/data/myapp"`, `"auth/biz/myapp"`
#[macro_export]
macro_rules! module_span {
    ($module:expr) => {
        tracing::info_span!("", module = $module)
    };
}

/// Pattern 2: Execute code within a module-tagged scope.
///
/// This is a convenience macro that creates a module span and executes
/// the provided expression within its scope.
///
/// # Examples
///
/// ```no_run
/// use lug::with_module;
///
/// with_module!("redis/data/myapp", {
///     tracing::info!("Connected to Redis");
///     // Multiple logs can appear here
/// });
/// ```
#[macro_export]
macro_rules! with_module {
    ($module:expr, $body:expr) => {{
        let _guard = $crate::module_span!($module).entered();
        $body
    }};
}

/// Pattern 3: Log an info message with a module tag.
///
/// # Examples
///
/// ```no_run
/// lug::info!(module: "auth/biz", user_id = 42, "Login successful");
/// ```
#[macro_export]
macro_rules! info {
    (module: $module:expr, $($arg:tt)*) => {
        tracing::info!(module = $module, $($arg)*)
    };
}

/// Pattern 3: Log a warning message with a module tag.
///
/// # Examples
///
/// ```no_run
/// lug::warn!(module: "database/data", latency_ms = 250, "Slow query detected");
/// ```
#[macro_export]
macro_rules! warn {
    (module: $module:expr, $($arg:tt)*) => {
        tracing::warn!(module = $module, $($arg)*)
    };
}

/// Pattern 3: Log an error message with a module tag.
///
/// # Examples
///
/// ```no_run
/// lug::error!(module: "auth/biz", "Authentication failed");
/// ```
#[macro_export]
macro_rules! error {
    (module: $module:expr, $($arg:tt)*) => {
        tracing::error!(module = $module, $($arg)*)
    };
}

/// Pattern 3: Log a debug message with a module tag.
///
/// # Examples
///
/// ```no_run
/// lug::debug!(module: "cache/data", key = "user:42", "Cache hit");
/// ```
#[macro_export]
macro_rules! debug {
    (module: $module:expr, $($arg:tt)*) => {
        tracing::debug!(module = $module, $($arg)*)
    };
}

/// Pattern 3: Log a trace message with a module tag.
///
/// # Examples
///
/// ```no_run
/// lug::trace!(module: "parser/core", token = "EOF", "Token parsed");
/// ```
#[macro_export]
macro_rules! trace {
    (module: $module:expr, $($arg:tt)*) => {
        tracing::trace!(module = $module, $($arg)*)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = LugConfig::default();
        assert_eq!(config.env, Environment::Prod);
        assert_eq!(config.level, Level::Info);
        assert!(config.file.is_none());
    }

    #[test]
    fn test_level_conversion() {
        assert_eq!(tracing::Level::from(Level::Debug), tracing::Level::DEBUG);
        assert_eq!(tracing::Level::from(Level::Info), tracing::Level::INFO);
        assert_eq!(tracing::Level::from(Level::Warn), tracing::Level::WARN);
        assert_eq!(tracing::Level::from(Level::Error), tracing::Level::ERROR);
    }
}
