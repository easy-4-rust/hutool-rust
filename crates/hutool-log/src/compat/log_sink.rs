use std::{
    collections::HashMap,
    fmt,
    sync::{Arc, OnceLock, RwLock},
};

use super::log_level::LogLevel;
use super::log_record::LogRecord;

/// Injectable destination used by all compatibility dialects.
pub trait LogSink: Send + Sync {
    /// Returns whether the destination accepts this category and level.
    fn enabled(&self, _name: &str, _level: LogLevel) -> bool {
        true
    }
    /// Emits one accepted record.
    fn emit(&self, record: &LogRecord);
}

use super::{format_message, global_slot};
