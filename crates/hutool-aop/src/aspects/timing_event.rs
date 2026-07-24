//! Hutool-aligned before/after aspects.

use crate::Method;
use parking_lot::Mutex;
use std::{
    any::type_name,
    collections::HashMap,
    fmt,
    sync::Arc,
    thread::{self, ThreadId},
    time::{Duration, Instant},
};

/// One completed timed invocation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimingEvent {
    /// Rust type name of the target.
    pub target_type: &'static str,
    /// Operation name.
    pub method: String,
    /// Measured wall-clock duration.
    pub elapsed: Duration,
    /// Debug-formatted return value, when present.
    pub return_value: Option<String>,
}
