//! Error types for the lug logging system.

use thiserror::Error;

/// Errors that can occur during logger initialization or operation.
#[derive(Debug, Error)]
pub enum LugError {
    /// Failed to initialize the global logger (e.g., already initialized)
    #[error("Failed to initialize logger: {0}")]
    InitFailed(String),

    /// Failed to set up file logging (e.g., permission denied)
    #[error("Failed to setup file logging: {0}")]
    FileSetupFailed(String),

    /// Invalid configuration provided
    #[error("Invalid configuration: {0}")]
    ConfigError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = LugError::InitFailed("already initialized".to_string());
        assert_eq!(
            err.to_string(),
            "Failed to initialize logger: already initialized"
        );

        let err = LugError::FileSetupFailed("permission denied".to_string());
        assert_eq!(
            err.to_string(),
            "Failed to setup file logging: permission denied"
        );
    }
}
