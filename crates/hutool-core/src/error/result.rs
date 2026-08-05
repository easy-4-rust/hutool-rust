//! Error types for core utilities.

/// 对齐: `cn.hutool.core.exceptions.CoreError`
/// 结果类型
use super::core_error::CoreError;

/// Result type returned by fallible core utilities.
pub type Result<T> = std::result::Result<T, CoreError>;
