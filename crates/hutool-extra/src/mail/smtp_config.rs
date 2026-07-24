//! Explicit, injectable SMTP and MIME mail support.

use std::{fmt, time::Duration};

pub use lettre::message::Mailbox;
use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    message::{Attachment, MultiPart, SinglePart, header::ContentType},
    transport::smtp::{authentication::Credentials, response::Response},
};
use secrecy::{ExposeSecret, SecretString};

use crate::{ExtraError, Result};

use super::smtp_credentials::SmtpCredentials;
use super::smtp_security::SmtpSecurity;

/// Complete SMTP connection configuration.
pub struct SmtpConfig {
    /// SMTP relay hostname.
    pub host: String,
    /// SMTP relay port.
    pub port: u16,
    /// Required channel security mode.
    pub security: SmtpSecurity,
    /// Optional authentication credentials.
    pub credentials: Option<SmtpCredentials>,
    /// Per-operation timeout.
    pub timeout: Duration,
}

impl fmt::Debug for SmtpConfig {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SmtpConfig")
            .field("host", &self.host)
            .field("port", &self.port)
            .field("security", &self.security)
            .field(
                "credentials",
                &self.credentials.as_ref().map(|_| "[REDACTED]"),
            )
            .field("timeout", &self.timeout)
            .finish()
    }
}
