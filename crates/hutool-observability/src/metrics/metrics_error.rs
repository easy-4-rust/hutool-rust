//! Metrics facade and an explicitly installed Prometheus recorder.

use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use thiserror::Error;

pub use metrics::{
    Counter, Gauge, Histogram, Unit, counter, describe_counter, describe_gauge, describe_histogram,
    gauge, histogram,
};

/// Metrics initialization failures.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum MetricsError {
    /// Another recorder is already installed or the recorder could not start.
    #[error("failed to install Prometheus recorder: {0}")]
    Install(String),
}
