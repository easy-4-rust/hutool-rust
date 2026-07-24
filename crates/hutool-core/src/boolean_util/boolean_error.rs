//! Boolean conversion and aggregation helpers aligned with Hutool.

use std::{any::TypeId, fmt};
use thiserror::Error;

/// Errors produced by boolean aggregations.
#[derive(Debug, Clone, Copy, Error, PartialEq, Eq)]
pub enum BooleanError {
    /// Hutool requires at least one operand for aggregate operations.
    #[error("boolean input must not be empty")]
    EmptyInput,
}
