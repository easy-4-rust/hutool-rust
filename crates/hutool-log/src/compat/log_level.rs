use std::{
    collections::HashMap,
    fmt,
    sync::{Arc, OnceLock, RwLock},
};

/// Hutool's five portable logging levels.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LogLevel {
    /// Highly detailed diagnostic events.
    #[default]
    Trace,
    /// Developer-oriented diagnostic events.
    Debug,
    /// Normal application events.
    Info,
    /// Recoverable or potentially harmful events.
    Warn,
    /// Failed operations.
    Error,
}

impl fmt::Display for LogLevel {
    /// Formats as the uppercase Hutool `Level` enum name (`DEBUG`, `INFO`, …).
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Trace => "TRACE",
            Self::Debug => "DEBUG",
            Self::Info => "INFO",
            Self::Warn => "WARN",
            Self::Error => "ERROR",
        })
    }
}

use super::{format_message, global_slot};
