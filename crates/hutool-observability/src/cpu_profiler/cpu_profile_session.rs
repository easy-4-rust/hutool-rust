use pprof::{ProfilerGuard, ProfilerGuardBuilder, protos::Message};
use thiserror::Error;

use crate::{AuthorizationError, DiagnosticAction, DiagnosticPermit};

use super::cpu_profile_config::CpuProfileConfig;
use super::cpu_profile_error::CpuProfileError;

/// Authorized in-process CPU profiling session.
pub struct CpuProfileSession {
    guard: ProfilerGuard<'static>,
}

impl std::fmt::Debug for CpuProfileSession {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("CpuProfileSession")
            .field("guard", &"<active>")
            .finish()
    }
}

impl CpuProfileSession {
    /// Starts CPU sampling after checking the runtime permit.
    pub fn start(
        config: &CpuProfileConfig,
        permit: &DiagnosticPermit,
    ) -> Result<Self, CpuProfileError> {
        permit.require(DiagnosticAction::CpuProfile)?;
        if !(1..=1_000).contains(&config.frequency) {
            return Err(CpuProfileError::InvalidFrequency(config.frequency));
        }
        let blocklist = config
            .blocklist
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>();
        let guard = ProfilerGuardBuilder::default()
            .frequency(config.frequency)
            .blocklist(&blocklist)
            .build()
            .map_err(|error| CpuProfileError::Backend(error.to_string()))?;
        Ok(Self { guard })
    }

    /// Builds a standard pprof protobuf payload.
    pub fn protobuf(&self, permit: &DiagnosticPermit) -> Result<Vec<u8>, CpuProfileError> {
        permit.require(DiagnosticAction::CpuProfile)?;
        let report = self
            .guard
            .report()
            .build()
            .map_err(|error| CpuProfileError::Backend(error.to_string()))?;
        let profile = report
            .pprof()
            .map_err(|error| CpuProfileError::Backend(error.to_string()))?;
        let mut output = Vec::new();
        profile
            .write_to_vec(&mut output)
            .map_err(|error| CpuProfileError::Encode(error.to_string()))?;
        Ok(output)
    }
}
