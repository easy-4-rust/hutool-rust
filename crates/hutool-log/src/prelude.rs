//! Re-exported convenience types for `hutool-log`.
//!
//! Usage:
//! ```rust
//! use hutool_log::prelude::*;
//! ```

pub use crate::{
    AbstractLog, Level, Log, LogFactory, LogLevel, LogRecord, LogSink, Redacted, StaticLog,
    TracingLog, init,
};
