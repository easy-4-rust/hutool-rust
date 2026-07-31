use serde::Serialize;

use super::health_status::HealthStatus;

/// Snapshot of one named health check.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HealthCheck {
    /// Current check status.
    pub status: HealthStatus,
    /// Optional operator-safe detail. Never put credentials or payloads here.
    pub detail: Option<String>,
    /// Last update time as milliseconds since the Unix epoch.
    pub updated_at_unix_ms: u64,
}
