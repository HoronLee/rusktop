//! Production environment configuration with file logging and rotation.

use lug::{Environment, FileConfig, Level, LugConfig};
use std::thread;
use std::time::Duration;

fn main() {
    // Create a temporary directory for this example
    let log_dir = std::env::temp_dir().join("lug_example");
    std::fs::create_dir_all(&log_dir).expect("Failed to create log directory");

    let log_file = log_dir.join("app.log");

    // Production configuration with file logging
    lug::init(LugConfig {
        env: Environment::Prod,
        level: Level::Info,
        file: Some(FileConfig {
            path: log_file.clone(),
            max_size_mb: 10,
            max_backups: 5,
            max_age_days: 7,
            compress: false,
        }),
    })
    .expect("Failed to initialize logger");

    println!("Logging to: {}", log_file.display());
    println!("Console output (non-colored) + JSON file output\n");

    // Simulate production logging
    tracing::info!("Application started");
    tracing::info!(
        version = "1.0.0",
        environment = "production",
        "Service configuration loaded"
    );

    // Simulate some work
    for i in 1..=5 {
        thread::sleep(Duration::from_millis(100));
        tracing::info!(
            request_id = i,
            latency_ms = i * 10,
            status = 200,
            "HTTP request processed"
        );
    }

    // Log with module tags
    let _guard = lug::module_span!("database/data/myapp").entered();
    tracing::info!(query = "SELECT * FROM users", "Database query executed");
    drop(_guard);

    tracing::warn!(memory_usage_mb = 256, "High memory usage detected");
    tracing::error!(
        error = "Connection timeout",
        "Failed to reach external service"
    );

    println!("\n✅ Logs written to: {}", log_file.display());
    println!("💡 Inspect the JSON file to see structured output");

    // Read and display a few lines from the log file
    if let Ok(content) = std::fs::read_to_string(&log_file) {
        println!("\n📄 Sample JSON log entries:\n");
        for (i, line) in content.lines().take(3).enumerate() {
            println!("Entry {}: {}", i + 1, line);
        }
    }

    println!("\n🗂️  Log directory: {}", log_dir.display());
}
