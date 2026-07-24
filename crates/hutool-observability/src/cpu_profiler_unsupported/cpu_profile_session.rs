use thiserror::Error;

use crate::{AuthorizationError, DiagnosticAction, DiagnosticPermit};

use super::cpu_profile_config::CpuProfileConfig;
use super::cpu_profile_error::CpuProfileError;

/// Uninhabited CPU profile session on non-Unix targets.
#[derive(Debug)]
pub struct CpuProfileSession {
    _private: (),
}

impl CpuProfileSession {
    /// Checks authorization and returns an unsupported-platform error.
    pub fn start(
        _config: &CpuProfileConfig,
        permit: &DiagnosticPermit,
    ) -> Result<Self, CpuProfileError> {
        permit.require(DiagnosticAction::CpuProfile)?;
        Err(CpuProfileError::UnsupportedPlatform)
    }

    /// Always returns an unsupported-platform error.
    pub fn protobuf(&self, permit: &DiagnosticPermit) -> Result<Vec<u8>, CpuProfileError> {
        permit.require(DiagnosticAction::CpuProfile)?;
        Err(CpuProfileError::UnsupportedPlatform)
    }
}
