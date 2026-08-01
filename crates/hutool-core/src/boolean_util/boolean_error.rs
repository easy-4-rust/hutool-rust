//! Boolean conversion and aggregation helpers aligned with Hutool.

/// 对齐: `cn.hutool.core.util.BooleanUtil`
/// 布尔值错误

use thiserror::Error;

/// Errors produced by boolean aggregations.
#[derive(Debug, Clone, Copy, Error, PartialEq, Eq)]
pub enum BooleanError {
    /// Hutool requires at least one operand for aggregate operations.
    #[error("boolean input must not be empty")]
    EmptyInput,
}
