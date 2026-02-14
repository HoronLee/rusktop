//! Basic usage example of the lug logger.

use lug::{Environment, Level, LugConfig};

fn main() {
    // Initialize the logger with dev configuration
    lug::init(LugConfig {
        env: Environment::Dev,
        level: Level::Debug,
        file: None,
    })
    .expect("Failed to initialize logger");

    // Use standard tracing macros
    tracing::info!("Application started");
    tracing::debug!(version = "0.1.0", "Debug information");
    tracing::warn!(latency_ms = 250, "Slow operation detected");

    // Structured logging with fields
    let user_id = 42;
    let username = "alice";
    tracing::info!(user_id = user_id, username = username, "User logged in");

    // Error logging
    let error_msg = "Connection refused";
    tracing::error!(error = error_msg, "Failed to connect to database");

    println!("\n✅ Check the colored output above!");
}
