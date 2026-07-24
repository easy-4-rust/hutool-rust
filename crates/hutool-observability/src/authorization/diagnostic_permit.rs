use std::{fmt, sync::Arc};

use subtle::ConstantTimeEq;
use thiserror::Error;
use zeroize::Zeroizing;

use super::authorization_error::AuthorizationError;
use super::diagnostic_action::DiagnosticAction;

/// Proof that an application authorization policy approved one diagnostic
/// action.
#[derive(Debug)]
pub struct DiagnosticPermit {
    pub(crate) action: DiagnosticAction,
}

impl DiagnosticPermit {
    /// Returns the diagnostic action authorized by this permit.
    #[must_use]
    pub fn action(&self) -> DiagnosticAction {
        self.action
    }

    #[cfg_attr(
        not(any(
            feature = "pprof",
            feature = "tokio-console",
            feature = "heap-profiler"
        )),
        allow(dead_code)
    )]
    pub(crate) fn require(&self, action: DiagnosticAction) -> Result<(), AuthorizationError> {
        if self.action == action {
            Ok(())
        } else {
            Err(AuthorizationError::WrongAction {
                expected: action,
                actual: self.action,
            })
        }
    }
}

use super::{DenyAll};
