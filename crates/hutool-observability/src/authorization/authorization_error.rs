use std::{fmt, sync::Arc};

use subtle::ConstantTimeEq;
use thiserror::Error;
use zeroize::Zeroizing;

use super::diagnostic_action::DiagnosticAction;

/// Diagnostic authorization failures.
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum AuthorizationError {
    /// Static diagnostic tokens must contain at least 128 bits of input.
    #[error("diagnostic token must contain at least 16 bytes")]
    WeakToken,
    /// The configured policy rejected the requested action.
    #[error("diagnostic action {0:?} was not authorized")]
    Denied(DiagnosticAction),
    /// A permit was presented for a different diagnostic action.
    #[error("permit authorizes {actual:?}, not {expected:?}")]
    WrongAction {
        /// Action required by the operation.
        expected: DiagnosticAction,
        /// Action contained in the permit.
        actual: DiagnosticAction,
    },
}

use super::{DenyAll};
