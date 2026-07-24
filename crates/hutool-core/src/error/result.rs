//! Error types for core utilities.

use super::core_error::CoreError;

/// Result type returned by fallible core utilities.
pub type Result<T> = std::result::Result<T, CoreError>;
