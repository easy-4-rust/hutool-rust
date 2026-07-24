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

/// Runtime-reloadable filter handle.
pub type FilterReloadHandle = Handle<EnvFilter, Registry>;

use super::{filter_from_value};
