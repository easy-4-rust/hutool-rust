//! Explicit, injectable SMTP and MIME mail support.

use std::fmt;

use secrecy::SecretString;

/// SMTP username and password stored in redacted containers.
pub struct SmtpCredentials {
    pub(crate) username: SecretString,
    pub(crate) password: SecretString,
}

impl SmtpCredentials {
    /// Creates SMTP credentials.
    #[must_use]
    pub fn new(username: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            username: SecretString::from(username.into()),
            password: SecretString::from(password.into()),
        }
    }
}

impl fmt::Debug for SmtpCredentials {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SmtpCredentials")
            .field("username", &"[REDACTED]")
            .field("password", &"[REDACTED]")
            .finish()
    }
}
