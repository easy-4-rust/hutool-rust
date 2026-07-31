//! Reloadable tracing configuration owned by the final application.

use thiserror::Error;

/// Tracing setup failures.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum TracingError {
    /// The filter expression is invalid.
    #[error("invalid tracing filter: {0}")]
    Filter(String),
    /// Another global subscriber is already installed.
    #[error("failed to install tracing subscriber: {0}")]
    Install(String),
    /// The reload layer is no longer active.
    #[error("failed to reload tracing filter: {0}")]
    Reload(String),
}
