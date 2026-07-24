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

/// SMTP channel security mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SmtpSecurity {
    /// TLS from connection establishment, normally port 465.
    Tls,
    /// Mandatory STARTTLS upgrade, normally port 587.
    StartTls,
    /// Unencrypted SMTP. Intended only for explicitly trusted local relays.
    Plaintext,
}
