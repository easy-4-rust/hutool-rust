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

/// Hutool `MailUtil` static helpers — build/send without global `GlobalMailAccount`.
///
/// 对齐 Java 类: `cn.hutool.extra.mail.MailUtil`
pub struct MailUtil;

impl MailUtil {
    /// Builds a plain-text [`MailMessage`] (Hutool `sendText` body path, no global SMTP).
    pub fn text(
        from: Mailbox,
        tos: impl IntoIterator<Item = Mailbox>,
        subject: impl Into<String>,
        content: impl Into<String>,
    ) -> Result<MailMessage> {
        let mut message = MailMessage::text(from, subject, content);
        let mut count = 0_usize;
        for recipient in tos {
            message = message.to(recipient);
            count += 1;
        }
        if count == 0 {
            return Err(ExtraError::InvalidMail(
                "at least one recipient is required",
            ));
        }
        Ok(message)
    }

    /// Builds an HTML [`MailMessage`] (Hutool `sendHtml` body path).
    pub fn html(
        from: Mailbox,
        tos: impl IntoIterator<Item = Mailbox>,
        subject: impl Into<String>,
        content: impl Into<String>,
    ) -> Result<MailMessage> {
        let mut message = MailMessage::html(from, subject, content);
        let mut count = 0_usize;
        for recipient in tos {
            message = message.to(recipient);
            count += 1;
        }
        if count == 0 {
            return Err(ExtraError::InvalidMail(
                "at least one recipient is required",
            ));
        }
        Ok(message)
    }

    /// Builds text or HTML from a boolean flag (Hutool `send(..., isHtml, ...)`).
    pub fn message(
        from: Mailbox,
        tos: impl IntoIterator<Item = Mailbox>,
        subject: impl Into<String>,
        content: impl Into<String>,
        is_html: bool,
    ) -> Result<MailMessage> {
        if is_html {
            Self::html(from, tos, subject, content)
        } else {
            Self::text(from, tos, subject, content)
        }
    }

    /// Sends a previously built message through an injected client (Hutool `send` transport).
    pub async fn send(client: &SmtpClient, message: &MailMessage) -> Result<Response> {
        client.send(message.build(MailLimits::default())?).await
    }

    /// Builds then sends using a [`MailAccount`] (Hutool `send(MailAccount, ...)`).
    pub async fn send_with_account(
        account: &MailAccount,
        message: &MailMessage,
    ) -> Result<Response> {
        let client = account.create_client()?;
        Self::send(&client, message).await
    }
}
