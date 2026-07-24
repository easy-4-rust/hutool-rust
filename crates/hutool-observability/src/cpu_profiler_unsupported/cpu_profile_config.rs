use thiserror::Error;

use crate::{AuthorizationError, DiagnosticAction, DiagnosticPermit};

/// CPU sampling configuration retained on unsupported platforms.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CpuProfileConfig {
    /// Requested samples per second.
    pub frequency: i32,
    /// Requested native-frame blocklist.
    pub blocklist: Vec<String>,
}

impl Default for CpuProfileConfig {
    fn default() -> Self {
        Self {
            frequency: 99,
            blocklist: Vec::new(),
        }
    }
}
