use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
    time::{SystemTime, UNIX_EPOCH},
};

use serde::Serialize;
use thiserror::Error;

use super::health_check::HealthCheck;
use super::health_status::HealthStatus;

/// Serializable aggregate health report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HealthReport {
    /// Application or service name.
    pub service: String,
    /// Worst status among all checks.
    pub status: HealthStatus,
    /// Individual checks ordered by name.
    pub checks: BTreeMap<String, HealthCheck>,
    /// Report creation time as milliseconds since the Unix epoch.
    pub generated_at_unix_ms: u64,
}

use super::{unix_time_ms};
