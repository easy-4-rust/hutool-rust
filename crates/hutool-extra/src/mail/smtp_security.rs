//! Explicit, injectable SMTP and MIME mail support.

/// SMTP channel security mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SmtpSecurity {
    /// TLS from connection establishment, normally port 465.
    Tls,
    /// Mandatory STARTTLS upgrade, normally port 587.
    StartTls,
    /// Unencrypted SMTP. Intended only for explicitly trusted local relays.
    Plaintext,
}
