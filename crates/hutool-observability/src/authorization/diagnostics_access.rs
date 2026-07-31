use std::{fmt, sync::Arc};

use super::authorization_error::AuthorizationError;
use super::diagnostic_action::DiagnosticAction;
use super::diagnostic_permit::DiagnosticPermit;
use super::diagnostics_authorizer::DiagnosticsAuthorizer;

/// Runtime access controller. Its default policy denies every diagnostic
/// operation.
#[derive(Clone)]
pub struct DiagnosticsAccess {
    authorizer: Arc<dyn DiagnosticsAuthorizer>,
}

impl fmt::Debug for DiagnosticsAccess {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DiagnosticsAccess")
            .field("authorizer", &"<policy>")
            .finish()
    }
}

impl Default for DiagnosticsAccess {
    fn default() -> Self {
        Self {
            authorizer: Arc::new(DenyAll),
        }
    }
}

impl DiagnosticsAccess {
    /// Creates an access controller from an application-owned policy.
    #[must_use]
    pub fn new(authorizer: impl DiagnosticsAuthorizer) -> Self {
        Self {
            authorizer: Arc::new(authorizer),
        }
    }

    /// Authorizes one diagnostic action and returns an unforgeable permit.
    pub fn authorize(
        &self,
        action: DiagnosticAction,
        credential: &[u8],
    ) -> Result<DiagnosticPermit, AuthorizationError> {
        if self.authorizer.authorize(action, credential) {
            Ok(DiagnosticPermit { action })
        } else {
            Err(AuthorizationError::Denied(action))
        }
    }
}

use super::{DenyAll};
