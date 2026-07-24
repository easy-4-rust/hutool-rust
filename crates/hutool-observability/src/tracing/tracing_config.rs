//! Reloadable tracing configuration owned by the final application.

use std::{env, ffi::OsString};

use thiserror::Error;
pub use tracing::{Level, debug, error, event, info, instrument, span, trace, warn};
use tracing_subscriber::{
    EnvFilter, Registry, fmt,
    layer::SubscriberExt,
    reload::{self, Handle},
    util::SubscriberInitExt,
};

/// Default text tracing configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TracingConfig {
    /// Filter used when `RUST_LOG` is absent or invalid.
    pub default_filter: String,
    /// Whether terminal output may contain ANSI color.
    pub ansi: bool,
}

impl Default for TracingConfig {
    fn default() -> Self {
        Self {
            default_filter: "info".to_owned(),
            ansi: true,
        }
    }
}

use super::{filter_from_value};
