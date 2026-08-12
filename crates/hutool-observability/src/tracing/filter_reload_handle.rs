//! Reloadable tracing configuration owned by the final application.

use tracing_subscriber::{EnvFilter, Registry, reload::Handle};

/// Runtime-reloadable filter handle.
pub type FilterReloadHandle = Handle<EnvFilter, Registry>;
