//! Layer construction utilities for file logging.

use crate::{FileConfig, LugError};
use std::path::Path;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::fmt::layer;
use tracing_subscriber::Layer;

/// Create a JSON file logging layer with rotation support.
///
/// Currently supports daily rotation. Full size-based rotation and
/// compression will be added in future versions.
///
/// # Errors
///
/// Returns `LugError::FileSetupFailed` if:
/// - Directory creation fails
/// - File path is invalid
pub fn create_file_layer<S>(
    config: &FileConfig,
) -> Result<Box<dyn Layer<S> + Send + Sync + 'static>, LugError>
where
    S: tracing::Subscriber,
    for<'a> S: tracing_subscriber::registry::LookupSpan<'a>,
{
    // Create log directory if it doesn't exist
    if let Some(parent) = config.path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| LugError::FileSetupFailed(format!("Failed to create directory: {}", e)))?;
    }

    // Determine rotation strategy
    // TODO: Implement size-based rotation when max_size_mb is set
    let rotation = match config.max_age_days {
        1 => Rotation::DAILY,
        7 => Rotation::DAILY, // Retention logic needs custom implementation
        _ => Rotation::DAILY, // Default to daily rotation
    };

    let directory = config.path.parent().unwrap_or_else(|| Path::new("."));

    let filename = config
        .path
        .file_name()
        .ok_or_else(|| LugError::FileSetupFailed("Invalid filename".to_string()))?;

    let file_appender = RollingFileAppender::new(rotation, directory, filename);

    // JSON format for structured logging
    Ok(Box::new(
        layer()
            .json()
            .with_writer(file_appender)
            .with_ansi(false)
            .with_timer(tracing_subscriber::fmt::time::ChronoLocal::rfc_3339()),
    ))
}

// TODO: Future enhancements
// 1. Size-based rotation: Implement custom MakeWriter that checks file size
//    Reference: https://github.com/tokio-rs/tracing/discussions/1877
//
// 2. Backup retention: Implement cleanup logic to respect max_backups
//    Can use a background task or on-write cleanup
//
// 3. Compression: Integrate with flate2 crate to compress old files
//    Trigger compression when rotating files
//
// Example implementation sketch:
// ```rust
// struct SizeRotatingWriter {
//     max_size: u64,
//     current_file: File,
//     rotation_callback: Box<dyn Fn(&Path)>,
// }
//
// impl std::io::Write for SizeRotatingWriter {
//     fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
//         if self.needs_rotation() {
//             self.rotate()?;
//         }
//         self.current_file.write(buf)
//     }
// }
// ```

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::TempDir;
    use tracing_subscriber::Registry;

    #[test]
    fn test_create_file_layer() {
        let temp_dir = TempDir::new().unwrap();
        let log_path = temp_dir.path().join("test.log");

        let config = FileConfig {
            path: log_path.clone(),
            ..Default::default()
        };

        let result = create_file_layer::<Registry>(&config);
        assert!(result.is_ok());

        // Verify directory was created
        assert!(log_path.parent().unwrap().exists());
    }

    #[test]
    fn test_invalid_filename() {
        let config = FileConfig {
            path: PathBuf::from("/"),
            ..Default::default()
        };

        let result = create_file_layer::<Registry>(&config);
        assert!(result.is_err());
    }
}
