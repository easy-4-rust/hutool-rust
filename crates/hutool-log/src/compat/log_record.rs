use std::{
    collections::HashMap,
    fmt,
    sync::{Arc, OnceLock, RwLock},
};

use super::log_level::LogLevel;

/// A backend-neutral logging event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogRecord {
    /// Logger/category name.
    pub name: String,
    /// Event severity.
    pub level: LogLevel,
    /// Rendered event message.
    pub message: String,
    /// Optional error description.
    pub error: Option<String>,
    /// Optional fully-qualified facade or caller name.
    pub fqcn: Option<String>,
}

impl LogRecord {
    /// Creates a record with owned data suitable for asynchronous sinks.
    #[must_use]
    pub fn new(name: &str, level: LogLevel, message: &str) -> Self {
        Self {
            name: name.to_owned(),
            level,
            message: message.to_owned(),
            error: None,
            fqcn: None,
        }
    }

    /// Attaches an error description.
    #[must_use]
    pub fn with_error(mut self, error: &str) -> Self {
        self.error = Some(error.to_owned());
        self
    }

    /// Attaches the original facade/caller name.
    #[must_use]
    pub fn with_fqcn(mut self, fqcn: &str) -> Self {
        self.fqcn = Some(fqcn.to_owned());
        self
    }
}

use super::{format_message, global_slot};
