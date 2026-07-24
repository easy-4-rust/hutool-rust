use thiserror::Error;

use crate::{AuthorizationError, DiagnosticAction, DiagnosticPermit};

/// CPU profiling failures on non-Unix targets.
#[derive(Debug, Error)]
pub enum CpuProfileError {
    /// The operation was not authorized.
    #[error(transparent)]
    Authorization(#[from] AuthorizationError),
    /// `pprof-rs` uses POSIX sampling and is unavailable on this target.
    #[error("in-process pprof CPU sampling is supported only on Unix targets")]
    UnsupportedPlatform,
}
