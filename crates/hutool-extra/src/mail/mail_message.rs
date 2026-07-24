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

use super::mail_attachment::MailAttachment;
use super::mail_body::MailBody;
use super::mail_limits::MailLimits;

/// Builder-style mail value with no global account or hidden session.
#[derive(Debug, Clone)]
pub struct MailMessage {
    from: Mailbox,
    to: Vec<Mailbox>,
    cc: Vec<Mailbox>,
    bcc: Vec<Mailbox>,
    reply_to: Vec<Mailbox>,
    subject: String,
    body: MailBody,
    attachments: Vec<MailAttachment>,
}

impl MailMessage {
    /// Creates a plain-text message.
    #[must_use]
    pub fn text(from: Mailbox, subject: impl Into<String>, body: impl Into<String>) -> Self {
        Self::new(from, subject.into(), MailBody::Text(body.into()))
    }

    /// Creates an HTML message.
    #[must_use]
    pub fn html(from: Mailbox, subject: impl Into<String>, body: impl Into<String>) -> Self {
        Self::new(from, subject.into(), MailBody::Html(body.into()))
    }

    fn new(from: Mailbox, subject: String, body: MailBody) -> Self {
        Self {
            from,
            to: Vec::new(),
            cc: Vec::new(),
            bcc: Vec::new(),
            reply_to: Vec::new(),
            subject,
            body,
            attachments: Vec::new(),
        }
    }

    /// Adds a primary recipient.
    #[must_use]
    pub fn to(mut self, recipient: Mailbox) -> Self {
        self.to.push(recipient);
        self
    }

    /// Adds a carbon-copy recipient.
    #[must_use]
    pub fn cc(mut self, recipient: Mailbox) -> Self {
        self.cc.push(recipient);
        self
    }

    /// Adds a blind-carbon-copy recipient.
    #[must_use]
    pub fn bcc(mut self, recipient: Mailbox) -> Self {
        self.bcc.push(recipient);
        self
    }

    /// Adds a reply-to address.
    #[must_use]
    pub fn reply_to(mut self, recipient: Mailbox) -> Self {
        self.reply_to.push(recipient);
        self
    }

    /// Adds an attachment or inline resource.
    #[must_use]
    pub fn attachment(mut self, attachment: MailAttachment) -> Self {
        self.attachments.push(attachment);
        self
    }

    /// Builds a standards-compliant MIME message after enforcing limits.
    pub fn build(&self, limits: MailLimits) -> Result<Message> {
        let recipient_count = self
            .to
            .len()
            .saturating_add(self.cc.len())
            .saturating_add(self.bcc.len());
        if recipient_count == 0 {
            return Err(ExtraError::InvalidMail(
                "at least one recipient is required",
            ));
        }
        if recipient_count > limits.max_recipients {
            return Err(ExtraError::InvalidMail("recipient limit exceeded"));
        }
        let body = match &self.body {
            MailBody::Text(body) | MailBody::Html(body) => body,
        };
        if body.len() > limits.max_body_bytes {
            return Err(ExtraError::InvalidMail("body byte limit exceeded"));
        }
        if self.attachments.len() > limits.max_attachments {
            return Err(ExtraError::MailAttachmentLimit("attachment count"));
        }
        let total = self
            .attachments
            .iter()
            .try_fold(0_usize, |total, attachment| {
                if attachment.bytes.len() > limits.max_attachment_bytes {
                    return Err(ExtraError::MailAttachmentLimit("single attachment bytes"));
                }
                total
                    .checked_add(attachment.bytes.len())
                    .ok_or(ExtraError::MailAttachmentLimit("total attachment bytes"))
            })?;
        if total > limits.max_total_attachment_bytes {
            return Err(ExtraError::MailAttachmentLimit("total attachment bytes"));
        }

        let mut builder = Message::builder()
            .from(self.from.clone())
            .subject(&self.subject);
        for recipient in &self.to {
            builder = builder.to(recipient.clone());
        }
        for recipient in &self.cc {
            builder = builder.cc(recipient.clone());
        }
        for recipient in &self.bcc {
            builder = builder.bcc(recipient.clone());
        }
        for recipient in &self.reply_to {
            builder = builder.reply_to(recipient.clone());
        }

        let body_part = match &self.body {
            MailBody::Text(body) => SinglePart::builder()
                .header(ContentType::TEXT_PLAIN)
                .body(body.clone()),
            MailBody::Html(body) => SinglePart::builder()
                .header(ContentType::TEXT_HTML)
                .body(body.clone()),
        };
        let mut multipart = MultiPart::mixed().singlepart(body_part);
        for attachment in &self.attachments {
            let part = if attachment.inline {
                Attachment::new_inline(attachment.name_or_cid.clone())
            } else {
                Attachment::new(attachment.name_or_cid.clone())
            }
            .body(attachment.bytes.clone(), attachment.content_type.clone());
            multipart = multipart.singlepart(part);
        }
        Ok(builder.multipart(multipart)?)
    }
}
