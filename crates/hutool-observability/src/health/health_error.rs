use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
    time::{SystemTime, UNIX_EPOCH},
};

use serde::Serialize;
use thiserror::Error;

/// Health registry failures.
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum HealthError {
    /// Service names must be explicit.
    #[error("health service name cannot be empty")]
    EmptyService,
    /// Check names must be explicit and bounded by the application.
    #[error("health check name cannot be empty")]
    EmptyCheck,
    /// Another thread panicked while holding the registry lock.
    #[error("health registry lock is poisoned")]
    Poisoned,
}

use super::{unix_time_ms};
