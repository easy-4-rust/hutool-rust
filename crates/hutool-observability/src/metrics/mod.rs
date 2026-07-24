//! Metrics facade and an explicitly installed Prometheus recorder.

use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use thiserror::Error;

pub use metrics::{
    Counter, Gauge, Histogram, Unit, counter, describe_counter, describe_gauge, describe_histogram,
    gauge, histogram,
};

mod prometheus_metrics;
mod metrics_error;

pub use prometheus_metrics::PrometheusMetrics;
pub use metrics_error::MetricsError;
