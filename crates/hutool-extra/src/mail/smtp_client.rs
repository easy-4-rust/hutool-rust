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

use super::smtp_config::SmtpConfig;
use super::smtp_security::SmtpSecurity;

/// Cloneable asynchronous SMTP client backed by Lettre and Rustls.
#[derive(Clone)]
pub struct SmtpClient {
    transport: AsyncSmtpTransport<Tokio1Executor>,
}

impl fmt::Debug for SmtpClient {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("SmtpClient").finish_non_exhaustive()
    }
}

impl SmtpClient {
    /// Builds a reusable SMTP connection pool.
    pub fn new(config: &SmtpConfig) -> Result<Self> {
        if config.host.trim().is_empty() || config.port == 0 || config.timeout.is_zero() {
            return Err(ExtraError::InvalidMail(
                "host, port and timeout are required",
            ));
        }
        let mut builder = match config.security {
            SmtpSecurity::Tls => AsyncSmtpTransport::<Tokio1Executor>::relay(&config.host)?,
            SmtpSecurity::StartTls => {
                AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&config.host)?
            }
            SmtpSecurity::Plaintext => {
                AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&config.host)
            }
        }
        .port(config.port)
        .timeout(Some(config.timeout));
        if let Some(credentials) = &config.credentials {
            builder = builder.credentials(Credentials::new(
                credentials.username.expose_secret().to_owned(),
                credentials.password.expose_secret().to_owned(),
            ));
        }
        Ok(Self {
            transport: builder.build(),
        })
    }

    /// Sends a previously bounded and built MIME message.
    pub async fn send(&self, message: Message) -> Result<Response> {
        Ok(self.transport.send(message).await?)
    }
}
