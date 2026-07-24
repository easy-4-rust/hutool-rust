//! Hutool-named mail facades over injectable SMTP/MIME helpers.
//!
//! 对齐: `cn.hutool.extra.mail.MailUtil`
//! 对齐: `cn.hutool.extra.mail.Mail`
//! 对齐: `cn.hutool.extra.mail.MailAccount`
//! 来源: hutool-extra/src/main/java/cn/hutool/extra/mail/

use std::time::Duration;

use lettre::message::{Mailbox, Message};
use lettre::transport::smtp::response::Response;

use crate::mail::{
    MailAttachment, MailBody, MailLimits, MailMessage, SmtpClient, SmtpConfig, SmtpCredentials,
    SmtpSecurity,
};
use crate::{ExtraError, Result};

use super::mail_account::MailAccount;

/// Hutool `Mail` builder — delegates to [`MailMessage`] (no global session).
///
/// 对齐 Java 类: `cn.hutool.extra.mail.Mail`
#[derive(Debug)]
pub struct Mail {
    account: Option<MailAccount>,
    from: Option<Mailbox>,
    to: Vec<Mailbox>,
    cc: Vec<Mailbox>,
    bcc: Vec<Mailbox>,
    reply: Vec<Mailbox>,
    title: String,
    content: String,
    html: bool,
    attachments: Vec<MailAttachment>,
}

impl Mail {
    /// Creates a mail builder with an account (Hutool `Mail.create(account)`).
    #[must_use]
    pub fn create(account: MailAccount) -> Self {
        let from = account.from().cloned();
        Self {
            account: Some(account),
            from,
            to: Vec::new(),
            cc: Vec::new(),
            bcc: Vec::new(),
            reply: Vec::new(),
            title: String::new(),
            content: String::new(),
            html: false,
            attachments: Vec::new(),
        }
    }

    /// Creates a mail builder without an account (Hutool `Mail.create()`).
    #[must_use]
    pub fn create_empty() -> Self {
        Self {
            account: None,
            from: None,
            to: Vec::new(),
            cc: Vec::new(),
            bcc: Vec::new(),
            reply: Vec::new(),
            title: String::new(),
            content: String::new(),
            html: false,
            attachments: Vec::new(),
        }
    }

    /// Sets primary recipients (Hutool `to` / `setTos`).
    #[must_use]
    pub fn to(mut self, recipients: impl IntoIterator<Item = Mailbox>) -> Self {
        self.to = recipients.into_iter().collect();
        self
    }

    /// Sets CC recipients (Hutool `setCcs`).
    #[must_use]
    pub fn set_ccs(mut self, recipients: impl IntoIterator<Item = Mailbox>) -> Self {
        self.cc = recipients.into_iter().collect();
        self
    }

    /// Sets BCC recipients (Hutool `setBccs`).
    #[must_use]
    pub fn set_bccs(mut self, recipients: impl IntoIterator<Item = Mailbox>) -> Self {
        self.bcc = recipients.into_iter().collect();
        self
    }

    /// Sets Reply-To addresses (Hutool `setReply`).
    #[must_use]
    pub fn set_reply(mut self, recipients: impl IntoIterator<Item = Mailbox>) -> Self {
        self.reply = recipients.into_iter().collect();
        self
    }

    /// Sets the From mailbox when not taken from [`MailAccount`].
    #[must_use]
    pub fn set_from(mut self, from: Mailbox) -> Self {
        self.from = Some(from);
        self
    }

    /// Sets the subject (Hutool `setTitle`).
    #[must_use]
    pub fn set_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Sets body text and HTML flag (Hutool `setContent` / `setHtml`).
    #[must_use]
    pub fn set_content(mut self, content: impl Into<String>, is_html: bool) -> Self {
        self.content = content.into();
        self.html = is_html;
        self
    }

    /// Marks the body as HTML (Hutool `setHtml`).
    #[must_use]
    pub fn set_html(mut self, is_html: bool) -> Self {
        self.html = is_html;
        self
    }

    /// Adds a file attachment (Hutool `setFiles` / `setAttachments` byte path).
    #[must_use]
    pub fn add_attachment(mut self, attachment: MailAttachment) -> Self {
        self.attachments.push(attachment);
        self
    }

    /// Adds an inline CID image (Hutool `addImage`).
    #[must_use]
    pub fn add_image(
        mut self,
        cid: impl Into<String>,
        content_type: &str,
        bytes: impl Into<Vec<u8>>,
    ) -> Result<Self> {
        self.attachments
            .push(MailAttachment::inline(cid, content_type, bytes)?);
        Ok(self)
    }

    /// Builds a [`MailMessage`] after validating From/To.
    pub fn to_message(&self) -> Result<MailMessage> {
        let from = self
            .from
            .clone()
            .ok_or(ExtraError::InvalidMail("from address is required"))?;
        let body = if self.html {
            MailBody::Html(self.content.clone())
        } else {
            MailBody::Text(self.content.clone())
        };
        let mut message = match body {
            MailBody::Text(text) => MailMessage::text(from, self.title.clone(), text),
            MailBody::Html(html) => MailMessage::html(from, self.title.clone(), html),
        };
        for recipient in &self.to {
            message = message.to(recipient.clone());
        }
        for recipient in &self.cc {
            message = message.cc(recipient.clone());
        }
        for recipient in &self.bcc {
            message = message.bcc(recipient.clone());
        }
        for recipient in &self.reply {
            message = message.reply_to(recipient.clone());
        }
        for attachment in &self.attachments {
            message = message.attachment(attachment.clone());
        }
        Ok(message)
    }

    /// Builds a MIME [`Message`] (Hutool `build`).
    pub fn build(&self) -> Result<Message> {
        self.to_message()?.build(MailLimits::default())
    }

    /// Sends via the account's client (Hutool `send` without a global session).
    pub async fn send(&self) -> Result<Response> {
        let account = self
            .account
            .as_ref()
            .ok_or(ExtraError::InvalidMail("mail account is required to send"))?;
        let client = account.create_client()?;
        client.send(self.build()?).await
    }
}
