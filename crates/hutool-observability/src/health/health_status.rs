use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
    time::{SystemTime, UNIX_EPOCH},
};

use serde::Serialize;
use thiserror::Error;

/// Health state ordered from best to worst.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HealthStatus {
    /// The component is fully operational.
    Healthy,
    /// The component is operational with reduced capability.
    Degraded,
    /// The component cannot serve its intended workload.
    Unhealthy,
}

impl HealthStatus {
    #[cfg(feature = "metrics")]
    pub(crate) fn metric_value(self) -> f64 {
        match self {
            Self::Healthy => 1.0,
            Self::Degraded => 0.5,
            Self::Unhealthy => 0.0,
        }
    }
}

use super::{unix_time_ms};
