use super::diagnostic_action::DiagnosticAction;

/// Application-supplied authorization policy for diagnostic operations.
pub trait DiagnosticsAuthorizer: Send + Sync + 'static {
    /// Returns `true` only when the credential may perform `action`.
    fn authorize(&self, action: DiagnosticAction, credential: &[u8]) -> bool;
}

impl DiagnosticsAuthorizer for DenyAll {
    fn authorize(&self, _action: DiagnosticAction, _credential: &[u8]) -> bool {
        false
    }
}

use super::DenyAll;
