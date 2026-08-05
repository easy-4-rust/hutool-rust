use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

use super::health_check::HealthCheck;
use super::health_error::HealthError;
use super::health_report::HealthReport;
use super::health_status::HealthStatus;

/// Thread-safe application-owned health registry.
#[derive(Debug, Clone)]
pub struct HealthRegistry {
    service: Arc<str>,
    checks: Arc<RwLock<BTreeMap<String, HealthCheck>>>,
}

impl HealthRegistry {
    /// Creates an empty registry.
    pub fn new(service: impl Into<String>) -> Result<Self, HealthError> {
        let service = service.into();
        if service.trim().is_empty() {
            return Err(HealthError::EmptyService);
        }
        Ok(Self {
            service: Arc::from(service),
            checks: Arc::new(RwLock::new(BTreeMap::new())),
        })
    }

    /// Inserts or replaces one bounded-cardinality health check.
    pub fn set(
        &self,
        name: impl Into<String>,
        status: HealthStatus,
        detail: Option<String>,
    ) -> Result<(), HealthError> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(HealthError::EmptyCheck);
        }
        let check = HealthCheck {
            status,
            detail,
            updated_at_unix_ms: unix_time_ms(),
        };
        self.checks
            .write()
            .map_err(|_| HealthError::Poisoned)?
            .insert(name.clone(), check);

        #[cfg(feature = "metrics")]
        metrics::gauge!(
            "hutool_health_check_status",
            "service" => self.service.to_string(),
            "check" => name.clone()
        )
        .set(status.metric_value());

        #[cfg(feature = "tracing")]
        tracing::info!(
            target: "hutool.observability.health",
            service = %self.service,
            check = %name,
            ?status,
            "health check updated"
        );

        Ok(())
    }

    /// Removes one check.
    pub fn remove(&self, name: &str) -> Result<Option<HealthCheck>, HealthError> {
        Ok(self
            .checks
            .write()
            .map_err(|_| HealthError::Poisoned)?
            .remove(name))
    }

    /// Returns a consistent health snapshot.
    pub fn report(&self) -> Result<HealthReport, HealthError> {
        let checks = self
            .checks
            .read()
            .map_err(|_| HealthError::Poisoned)?
            .clone();
        let status = checks
            .values()
            .map(|check| check.status)
            .max()
            .unwrap_or(HealthStatus::Healthy);
        Ok(HealthReport {
            service: self.service.to_string(),
            status,
            checks,
            generated_at_unix_ms: unix_time_ms(),
        })
    }

    /// Returns `true` when no check is unhealthy.
    pub fn is_ready(&self) -> Result<bool, HealthError> {
        Ok(self.report()?.status != HealthStatus::Unhealthy)
    }
}

use super::unix_time_ms;
