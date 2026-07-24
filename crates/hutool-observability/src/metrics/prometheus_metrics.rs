//! Metrics facade and an explicitly installed Prometheus recorder.

use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use thiserror::Error;

pub use metrics::{
    Counter, Gauge, Histogram, Unit, counter, describe_counter, describe_gauge, describe_histogram,
    gauge, histogram,
};

use super::metrics_error::MetricsError;

/// Handle for rendering the process-wide Prometheus recorder.
#[derive(Debug, Clone)]
pub struct PrometheusMetrics {
    handle: PrometheusHandle,
}

impl PrometheusMetrics {
    /// Installs the process-wide recorder.
    ///
    /// This function is intentionally explicit because the metrics recorder
    /// can only be installed once. It does not start a scrape server.
    pub fn install() -> Result<Self, MetricsError> {
        let handle = PrometheusBuilder::new()
            .with_recommended_naming(true)
            .install_recorder()
            .map_err(|error| MetricsError::Install(error.to_string()))?;
        Ok(Self { handle })
    }

    /// Renders a Prometheus text exposition payload.
    #[must_use]
    pub fn render(&self) -> String {
        self.handle.render()
    }
}
