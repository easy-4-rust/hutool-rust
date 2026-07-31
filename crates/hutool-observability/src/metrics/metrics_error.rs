//! Metrics facade and an explicitly installed Prometheus recorder.

use thiserror::Error;

/// Metrics initialization failures.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum MetricsError {
    /// Another recorder is already installed or the recorder could not start.
    #[error("failed to install Prometheus recorder: {0}")]
    Install(String),
}
