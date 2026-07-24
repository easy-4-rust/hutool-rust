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

/// Plain-text or HTML message body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MailBody {
    /// UTF-8 plain text.
    Text(String),
    /// UTF-8 HTML. Content sanitization remains the caller's responsibility.
    Html(String),
}
