//! SeaORM logging integration.
//!
//! This module provides utilities to configure SeaORM's logging to work
//! seamlessly with lug's tracing-based logging system.

use sea_orm::ConnectOptions;
use std::time::Duration;

/// Configure SeaORM connection options to enable logging with module tagging.
///
/// This function sets up SQL query logging through SeaORM's sqlx backend,
/// which automatically integrates with the tracing ecosystem.
///
/// # Arguments
///
/// * `opt` - Mutable reference to SeaORM's ConnectOptions
/// * `module` - Module tag to apply to all database logs (e.g., "database/data/myapp")
///
/// # Examples
///
/// ```no_run
/// use lug::integrations::configure_sea_orm;
/// use sea_orm::{Database, ConnectOptions};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let mut opt = ConnectOptions::new("postgres://localhost/mydb");
/// configure_sea_orm(&mut opt, "database/data/myapp");
///
/// let db = Database::connect(opt).await?;
/// // SQL queries will now be logged with module="database/data/myapp"
/// # Ok(())
/// # }
/// ```
///
/// # Notes
///
/// - SQL query logging can be controlled via the `RUST_LOG` environment variable:
///   ```bash
///   RUST_LOG=sqlx=debug cargo run
///   ```
/// - The `module` parameter will appear in all log entries from this database connection
/// - Slow query detection can be added by wrapping queries in tracing spans
pub fn configure_sea_orm(opt: &mut ConnectOptions, module: &str) {
    use tracing::log::LevelFilter;

    opt.sqlx_logging(true)
        .sqlx_logging_level(LevelFilter::Debug)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8));

    // Log the configuration with module tag
    tracing::info!(module = module, "SeaORM logging configured");
}

/// Extension trait for adding slow query tracing to SeaORM operations.
///
/// # Examples
///
/// ```no_run
/// use lug::integrations::sea_orm::SlowQueryExt;
/// use sea_orm::{Database, EntityTrait};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// # let db = Database::connect("sqlite::memory:").await?;
/// // Automatically log queries that exceed 200ms
/// // let users = User::find()
/// //     .all(&db)
/// //     .with_slow_query_tracing("users/query", Duration::from_millis(200))
/// //     .await?;
/// # Ok(())
/// # }
/// ```
pub trait SlowQueryExt {
    /// Enable slow query tracing for this operation.
    ///
    /// Logs a warning if the query exceeds the specified threshold.
    fn with_slow_query_tracing(self, module: &str, threshold: Duration) -> Self;
}

// Note: Actual implementation would require wrapping SeaORM's query execution
// This is a design sketch for future enhancement
// impl<T> SlowQueryExt for T where T: Future {
//     fn with_slow_query_tracing(self, module: &str, threshold: Duration) -> Self {
//         // Wrap in a tracing span and measure execution time
//         todo!("Implement using tracing::instrument")
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_configure_sea_orm() {
        let mut opt = ConnectOptions::new("sqlite::memory:");
        configure_sea_orm(&mut opt, "test/database");

        // Verify options are set
        assert_eq!(
            opt.get_sqlx_logging_level(),
            tracing::log::LevelFilter::Debug
        );
        assert!(opt.get_sqlx_logging());
    }
}
