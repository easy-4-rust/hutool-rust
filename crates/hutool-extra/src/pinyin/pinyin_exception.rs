//! Pinyin helpers aligned with Hutool `PinyinUtil`, backed by the `pinyin` crate.

use pinyin::{ToPinyin, ToPinyinMulti};

/// Error type matching Hutool `PinyinException` messaging.
#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
#[error("{message}")]
pub struct PinyinException {
    message: String,
}

impl PinyinException {
    /// Java: `new PinyinException(String message)`
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    /// Java: `new PinyinException(Throwable)`
    #[must_use]
    pub fn from_cause(cause: impl std::fmt::Display) -> Self {
        Self {
            message: cause.to_string(),
        }
    }

    /// Returns the error message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}
