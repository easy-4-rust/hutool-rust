//! Explicit, injectable SMTP and MIME mail support.
//!
//! 对齐: `cn.hutool.extra.mail.MailUtil`
//! 邮件工具类

pub use lettre::message::Mailbox;

mod mail_attachment;
mod mail_body;
mod mail_limits;
mod mail_message;
mod smtp_client;
mod smtp_config;
mod smtp_credentials;
mod smtp_security;

pub use mail_attachment::MailAttachment;
pub use mail_body::MailBody;
pub use mail_limits::MailLimits;
pub use mail_message::MailMessage;
pub use smtp_client::SmtpClient;
pub use smtp_config::SmtpConfig;
pub use smtp_credentials::SmtpCredentials;
pub use smtp_security::SmtpSecurity;
