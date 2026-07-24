use pprof::{ProfilerGuard, ProfilerGuardBuilder, protos::Message};
use thiserror::Error;

use crate::{AuthorizationError, DiagnosticAction, DiagnosticPermit};

/// CPU sampling configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CpuProfileConfig {
    /// Samples per second. Values from 1 through 1,000 are accepted.
    pub frequency: i32,
    /// Native frames excluded from stack collection.
    pub blocklist: Vec<String>,
}

impl Default for CpuProfileConfig {
    fn default() -> Self {
        Self {
            frequency: 99,
            blocklist: vec![
                "libc".to_owned(),
                "libgcc".to_owned(),
                "pthread".to_owned(),
                "vdso".to_owned(),
            ],
        }
    }
}
