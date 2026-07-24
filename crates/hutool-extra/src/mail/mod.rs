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

mod smtp_security;
mod smtp_credentials;
mod smtp_config;
mod mail_limits;
mod mail_body;
mod mail_attachment;
mod mail_message;
mod smtp_client;

pub use smtp_security::SmtpSecurity;
pub use smtp_credentials::SmtpCredentials;
pub use smtp_config::SmtpConfig;
pub use mail_limits::MailLimits;
pub use mail_body::MailBody;
pub use mail_attachment::MailAttachment;
pub use mail_message::MailMessage;
pub use smtp_client::SmtpClient;
