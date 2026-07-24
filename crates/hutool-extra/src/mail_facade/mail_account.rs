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

use super::mail::Mail;

/// Hutool `MailAccount` — SMTP relay settings without a global singleton.
///
/// 对齐 Java 类: `cn.hutool.extra.mail.MailAccount`
#[derive(Debug)]
pub struct MailAccount {
    inner: SmtpConfig,
    from: Option<Mailbox>,
}

impl MailAccount {
    /// Creates an empty account (Hutool no-arg constructor defaults filled later).
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: SmtpConfig {
                host: String::new(),
                port: 465,
                security: SmtpSecurity::Tls,
                credentials: None,
                timeout: Duration::from_secs(30),
            },
            from: None,
        }
    }

    /// Creates an account from an existing SMTP config.
    #[must_use]
    pub fn from_config(config: SmtpConfig) -> Self {
        Self {
            inner: config,
            from: None,
        }
    }

    /// Sets SMTP host (Hutool `setHost`).
    #[must_use]
    pub fn set_host(mut self, host: impl Into<String>) -> Self {
        self.inner.host = host.into();
        self
    }

    /// Returns SMTP host (Hutool `getHost`).
    #[must_use]
    pub fn host(&self) -> &str {
        &self.inner.host
    }

    /// Sets SMTP port (Hutool `setPort`).
    #[must_use]
    pub fn set_port(mut self, port: u16) -> Self {
        self.inner.port = port;
        self
    }

    /// Returns SMTP port (Hutool `getPort`).
    #[must_use]
    pub fn port(&self) -> u16 {
        self.inner.port
    }

    /// Enables TLS / STARTTLS style security (Hutool `setSslEnable` / `setStarttlsEnable`).
    #[must_use]
    pub fn set_ssl_enable(mut self, enable: bool) -> Self {
        self.inner.security = if enable {
            SmtpSecurity::Tls
        } else {
            SmtpSecurity::Plaintext
        };
        self
    }

    /// Forces STARTTLS (Hutool `setStarttlsEnable(true)`).
    #[must_use]
    pub fn set_starttls_enable(mut self, enable: bool) -> Self {
        if enable {
            self.inner.security = SmtpSecurity::StartTls;
        }
        self
    }

    /// Sets username/password (Hutool `setUser` / `setPass` / `setAuth`).
    #[must_use]
    pub fn set_auth(mut self, user: impl Into<String>, pass: impl Into<String>) -> Self {
        self.inner.credentials = Some(SmtpCredentials::new(user, pass));
        self
    }

    /// Sets the From mailbox used by [`Mail`] builders.
    #[must_use]
    pub fn set_from(mut self, from: Mailbox) -> Self {
        self.from = Some(from);
        self
    }

    /// Returns the configured From mailbox when set.
    #[must_use]
    pub fn from(&self) -> Option<&Mailbox> {
        self.from.as_ref()
    }

    /// Borrows the underlying SMTP configuration.
    #[must_use]
    pub fn smtp_config(&self) -> &SmtpConfig {
        &self.inner
    }

    /// Builds an injectable [`SmtpClient`] (Hutool session factory without globals).
    pub fn create_client(&self) -> Result<SmtpClient> {
        SmtpClient::new(&self.inner)
    }
}

impl Default for MailAccount {
    fn default() -> Self {
        Self::new()
    }
}
