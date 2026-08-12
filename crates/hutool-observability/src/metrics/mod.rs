//! Metrics facade and an explicitly installed Prometheus recorder.

pub use metrics::{
    Counter, Gauge, Histogram, Unit, counter, describe_counter, describe_gauge, describe_histogram,
    gauge, histogram,
};

mod metrics_error;
mod prometheus_metrics;

pub use metrics_error::MetricsError;
pub use prometheus_metrics::PrometheusMetrics;
