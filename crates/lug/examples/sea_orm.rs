//! SeaORM integration example (requires feature: sea-orm-integration)

#[cfg(feature = "sea-orm-integration")]
use lug::integrations::configure_sea_orm;
#[cfg(feature = "sea-orm-integration")]
use sea_orm::{ConnectOptions, Database};

#[cfg(feature = "sea-orm-integration")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize lug logger
    lug::init(lug::LugConfig::default())?;

    println!("=== SeaORM Logging Integration Example ===\n");

    // Configure SeaORM connection with logging
    let mut opt = ConnectOptions::new("sqlite::memory:");
    configure_sea_orm(&mut opt, "database/data/myapp");

    // Connect to database
    let db = Database::connect(opt).await?;

    println!("✅ Connected to database with logging enabled");
    println!("💡 Set RUST_LOG=sqlx=debug to see SQL queries\n");

    // Example: SQL queries will be automatically logged
    // Note: Actual table creation would require SeaORM entity definitions
    println!("📝 All SQL queries executed through this connection will be logged");
    println!("   with module=\"database/data/myapp\"\n");

    Ok(())
}

#[cfg(not(feature = "sea-orm-integration"))]
fn main() {
    eprintln!("❌ This example requires the 'sea-orm-integration' feature");
    eprintln!("   Run with: cargo run --example sea_orm_example --features sea-orm-integration");
    std::process::exit(1);
}
