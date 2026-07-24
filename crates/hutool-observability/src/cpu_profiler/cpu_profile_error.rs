use pprof::{ProfilerGuard, ProfilerGuardBuilder, protos::Message};
use thiserror::Error;

use crate::{AuthorizationError, DiagnosticAction, DiagnosticPermit};

/// CPU profiling failures.
#[derive(Debug, Error)]
pub enum CpuProfileError {
    /// The operation was not authorized.
    #[error(transparent)]
    Authorization(#[from] AuthorizationError),
    /// Sampling frequencies outside the supported safety range are rejected.
    #[error("CPU profile frequency must be between 1 and 1000 Hz, got {0}")]
    InvalidFrequency(i32),
    /// The native sampler failed.
    #[error("CPU profiler failed: {0}")]
    Backend(String),
    /// The report could not be encoded.
    #[error("CPU profile encoding failed: {0}")]
    Encode(String),
}
