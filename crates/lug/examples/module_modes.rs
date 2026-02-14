//! Demonstrates three different patterns for adding module tags to logs.

use lug::{module_span, with_module, Environment, LugConfig};

fn main() {
    // Initialize logger
    lug::init(LugConfig {
        env: Environment::Dev,
        ..Default::default()
    })
    .expect("Failed to initialize logger");

    println!("=== Pattern 1: Span Mode (Recommended) ===\n");
    pattern_1_span_mode();

    println!("\n=== Pattern 2: Scoped Macro ===\n");
    pattern_2_scoped_macro();

    println!("\n=== Pattern 3: Inline Macro ===\n");
    pattern_3_inline_macro();

    println!("\n✅ All patterns demonstrated!");
}

/// Pattern 1: Using module_span! for automatic context propagation
fn pattern_1_span_mode() {
    // Create a span with module tag
    let _guard = module_span!("auth/biz/myapp").entered();

    // All logs within this scope automatically include module="auth/biz/myapp"
    tracing::info!(user_id = 42, "User authentication started");
    tracing::debug!("Validating credentials");

    // Nested function calls also inherit the module tag
    check_permissions();

    tracing::info!("Authentication successful");
    // Guard is dropped here, ending the span
}

fn check_permissions() {
    // This log inherits module="auth/biz/myapp" from parent span
    tracing::debug!("Checking user permissions");
}

/// Pattern 2: Using with_module! for scoped execution
fn pattern_2_scoped_macro() {
    with_module!("redis/data/myapp", {
        tracing::info!("Connecting to Redis");
        tracing::debug!(host = "localhost", port = 6379, "Connection parameters");
        tracing::info!("Connected successfully");
    });

    // Multiple scopes with different modules
    with_module!("postgres/data/myapp", {
        tracing::info!("Executing database query");
    });
}

/// Pattern 3: Using inline macros for one-off logs
fn pattern_3_inline_macro() {
    // Each log specifies its own module
    lug::info!(module: "metrics/api/myapp", counter = 100, "Request processed");
    lug::warn!(module: "cache/data/myapp", hit_rate = 0.85, "Low cache hit rate");
    lug::error!(module: "payment/biz/myapp", "Payment processing failed");
    lug::debug!(module: "parser/core/myapp", token = "EOF", "Token parsed");
    lug::trace!(module: "network/io/myapp", "Packet received");
}
