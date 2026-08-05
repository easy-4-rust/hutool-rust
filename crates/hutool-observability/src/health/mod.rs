use std::time::{SystemTime, UNIX_EPOCH};

mod health_check;
mod health_error;
mod health_registry;
mod health_report;
mod health_status;

pub use health_check::HealthCheck;
pub use health_error::HealthError;
pub use health_registry::HealthRegistry;
pub use health_report::HealthReport;
pub use health_status::HealthStatus;

fn unix_time_ms() -> u64 {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    u64::try_from(millis).unwrap_or(u64::MAX)
}
