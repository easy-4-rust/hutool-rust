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

/// Defensive MIME construction limits.
#[derive(Debug, Clone, Copy)]
pub struct MailLimits {
    /// Maximum total To/Cc/Bcc recipients.
    pub max_recipients: usize,
    /// Maximum UTF-8 body bytes.
    pub max_body_bytes: usize,
    /// Maximum attachment count.
    pub max_attachments: usize,
    /// Maximum bytes in one attachment.
    pub max_attachment_bytes: usize,
    /// Maximum bytes across all attachments.
    pub max_total_attachment_bytes: usize,
}

impl Default for MailLimits {
    fn default() -> Self {
        Self {
            max_recipients: 100,
            max_body_bytes: 2 * 1024 * 1024,
            max_attachments: 32,
            max_attachment_bytes: 25 * 1024 * 1024,
            max_total_attachment_bytes: 50 * 1024 * 1024,
        }
    }
}
