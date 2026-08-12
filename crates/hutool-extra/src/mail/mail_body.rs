//! Explicit, injectable SMTP and MIME mail support.

/// Plain-text or HTML message body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MailBody {
    /// UTF-8 plain text.
    Text(String),
    /// UTF-8 HTML. Content sanitization remains the caller's responsibility.
    Html(String),
}
