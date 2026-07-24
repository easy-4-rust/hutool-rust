use std::{fmt, sync::Arc};

use subtle::ConstantTimeEq;
use thiserror::Error;
use zeroize::Zeroizing;

/// Diagnostic capability protected by runtime authorization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiagnosticAction {
    /// Start or read an in-process CPU profile.
    CpuProfile,
    /// Start the Tokio console telemetry server.
    TokioConsole,
    /// Start an in-process heap profile.
    HeapProfile,
}

use super::{DenyAll};
