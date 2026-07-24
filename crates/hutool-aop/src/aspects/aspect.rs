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

/// A typed Hutool-compatible aspect.
///
/// Returning `false` from `before` skips the operation. Returning `false`
/// from `after` suppresses its value. Returning `true` from
/// `after_exception` allows the error to propagate.
pub trait Aspect<T, A, R, E>: Send + Sync {
    /// Runs before the target and decides whether it may execute.
    fn before(&self, _target: &T, _method: &Method, _args: &A) -> bool {
        true
    }

    /// Runs after a successful or deliberately suppressed invocation.
    fn after(&self, _target: &T, _method: &Method, _args: &A, _return_value: Option<&R>) -> bool {
        true
    }

    /// Runs after a target error and decides whether it propagates.
    fn after_exception(&self, _target: &T, _method: &Method, _args: &A, _error: &E) -> bool {
        true
    }
}
