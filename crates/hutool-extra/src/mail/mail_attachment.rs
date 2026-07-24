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

/// Owned MIME attachment or inline resource.
#[derive(Debug, Clone)]
pub struct MailAttachment {
    name_or_cid: String,
    content_type: ContentType,
    bytes: Vec<u8>,
    inline: bool,
}

impl MailAttachment {
    /// Creates a downloadable attachment.
    pub fn file(
        filename: impl Into<String>,
        content_type: &str,
        bytes: impl Into<Vec<u8>>,
    ) -> Result<Self> {
        Self::new(filename.into(), content_type, bytes.into(), false)
    }

    /// Creates an inline resource referenced with `cid:<content_id>`.
    pub fn inline(
        content_id: impl Into<String>,
        content_type: &str,
        bytes: impl Into<Vec<u8>>,
    ) -> Result<Self> {
        Self::new(content_id.into(), content_type, bytes.into(), true)
    }

    fn new(name_or_cid: String, content_type: &str, bytes: Vec<u8>, inline: bool) -> Result<Self> {
        if name_or_cid.trim().is_empty() {
            return Err(ExtraError::InvalidMail(
                "attachment name/content id is empty",
            ));
        }
        let content_type = ContentType::parse(content_type)
            .map_err(|_| ExtraError::InvalidMail("attachment content type is invalid"))?;
        Ok(Self {
            name_or_cid,
            content_type,
            bytes,
            inline,
        })
    }
}
