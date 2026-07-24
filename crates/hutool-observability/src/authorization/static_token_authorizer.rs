use std::{fmt, sync::Arc};

use subtle::ConstantTimeEq;
use thiserror::Error;
use zeroize::Zeroizing;

use super::authorization_error::AuthorizationError;
use super::diagnostic_action::DiagnosticAction;
use super::diagnostics_authorizer::DiagnosticsAuthorizer;

/// Constant-time shared-token authorizer suitable for protected management
/// endpoints and local diagnostic tooling.
pub struct StaticTokenAuthorizer {
    token: Zeroizing<Vec<u8>>,
}

impl fmt::Debug for StaticTokenAuthorizer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("StaticTokenAuthorizer")
            .field("token", &"[REDACTED]")
            .finish()
    }
}

impl StaticTokenAuthorizer {
    /// Creates an authorizer. Tokens shorter than 16 bytes are rejected.
    pub fn new(token: impl Into<Vec<u8>>) -> Result<Self, AuthorizationError> {
        let token = token.into();
        if token.len() < 16 {
            return Err(AuthorizationError::WeakToken);
        }
        Ok(Self {
            token: Zeroizing::new(token),
        })
    }
}

impl DiagnosticsAuthorizer for StaticTokenAuthorizer {
    fn authorize(&self, _action: DiagnosticAction, credential: &[u8]) -> bool {
        self.token.len() == credential.len() && bool::from(self.token.as_slice().ct_eq(credential))
    }
}

use super::{DenyAll};
