use std::{fmt, sync::Arc};

use subtle::ConstantTimeEq;
use thiserror::Error;
use zeroize::Zeroizing;

mod diagnostic_action;
mod diagnostics_authorizer;
mod static_token_authorizer;
mod diagnostics_access;
mod diagnostic_permit;
mod authorization_error;

pub use diagnostic_action::DiagnosticAction;
pub use diagnostics_authorizer::DiagnosticsAuthorizer;
pub use static_token_authorizer::StaticTokenAuthorizer;
pub use diagnostics_access::DiagnosticsAccess;
pub use diagnostic_permit::DiagnosticPermit;
pub use authorization_error::AuthorizationError;

struct DenyAll;
