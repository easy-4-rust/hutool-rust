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

mod filter_reload_handle;
mod tracing_config;
mod tracing_error;

pub use filter_reload_handle::FilterReloadHandle;
pub use tracing_config::TracingConfig;
pub use tracing_error::TracingError;

pub fn install(config: &TracingConfig) -> Result<FilterReloadHandle, TracingError> {
    let (filter, handle) = reloadable_filter(config)?;
    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().compact().with_ansi(config.ansi))
        .try_init()
        .map_err(|error| TracingError::Install(error.to_string()))?;
    Ok(handle)
}

pub fn reloadable_filter(
    config: &TracingConfig,
) -> Result<(reload::Layer<EnvFilter, Registry>, FilterReloadHandle), TracingError> {
    let filter = filter_from_value(env::var_os("RUST_LOG"), &config.default_filter)?;
    Ok(reload::Layer::new(filter))
}

pub fn reload_filter(handle: &FilterReloadHandle, filter: &str) -> Result<(), TracingError> {
    let filter =
        EnvFilter::try_new(filter).map_err(|error| TracingError::Filter(error.to_string()))?;
    handle
        .reload(filter)
        .map_err(|error| TracingError::Reload(error.to_string()))
}

fn filter_from_value(
    value: Option<OsString>,
    default_filter: &str,
) -> Result<EnvFilter, TracingError> {
    if let Some(value) = value.and_then(|value| value.into_string().ok()) {
        if let Ok(filter) = EnvFilter::try_new(value) {
            return Ok(filter);
        }
    }
    EnvFilter::try_new(default_filter).map_err(|error| TracingError::Filter(error.to_string()))
}
