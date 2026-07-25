//! 对齐: `cn.hutool.log.Log` (日志门面模块)
//! 来源: hutool-log/src/main/java/cn/hutool/log/
//! 中文说明: 基于 Rust `tracing` 生态的结构化应用日志模块，提供与 Hutool Java 日志体系兼容的抽象层。
//!
//! Structured application logging built on the Rust `tracing` ecosystem.

#![forbid(unsafe_code)]

use std::fmt as std_fmt;
use std::{env, ffi::OsString};
pub use tracing::{Level, debug, error, event, info, instrument, span, trace, warn};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

mod compat;

pub use compat::{
    AbstractLog, GlobalLogFactory, Log, LogFactory, LogLevel, LogRecord, LogSink, StaticLog,
    TracingLog, TracingSink, format_message,
};

/// Hutool-compatible log levels without colliding with `tracing::Level`.
pub mod level {
    pub use crate::compat::LogLevel as Level;
}

/// Backend names retained as compatibility adapters over the common tracing engine.
#[allow(missing_docs)]
pub mod dialect {
    pub mod commons {
        pub type ApacheCommonsLog = crate::TracingLog;
        pub type ApacheCommonsLogFactory = crate::LogFactory;
        pub type ApacheCommonsLog4JLog = crate::TracingLog;
    }
    pub mod console {
        pub type ConsoleLog = crate::TracingLog;
        pub type ConsoleLogFactory = crate::LogFactory;
        pub type ConsoleColorLog = crate::TracingLog;
        pub type ConsoleColorLogFactory = crate::LogFactory;
    }
    pub mod jboss {
        pub type JbossLog = crate::TracingLog;
        pub type JbossLogFactory = crate::LogFactory;
    }
    pub mod jdk {
        pub type JdkLog = crate::TracingLog;
        pub type JdkLogFactory = crate::LogFactory;
    }
    pub mod log4j {
        pub type Log4jLog = crate::TracingLog;
        pub type Log4jLogFactory = crate::LogFactory;
    }
    pub mod log4j2 {
        pub type Log4j2Log = crate::TracingLog;
        pub type Log4j2LogFactory = crate::LogFactory;
    }
    pub mod logtube {
        pub type LogTubeLog = crate::TracingLog;
        pub type LogTubeLogFactory = crate::LogFactory;
    }
    pub mod slf4j {
        pub type Slf4jLog = crate::TracingLog;
        pub type Slf4jLogFactory = crate::LogFactory;
    }
    pub mod tinylog {
        pub type TinyLog = crate::TracingLog;
        pub type TinyLogFactory = crate::LogFactory;
        pub type TinyLog2 = crate::TracingLog;
        pub type TinyLog2Factory = crate::LogFactory;
    }
}

/// Installs a compact global text subscriber.
///
/// The `RUST_LOG` environment variable takes precedence over `default_filter`.
pub fn init(default_filter: &str) -> Result<(), tracing_subscriber::util::TryInitError> {
    let filter = filter_from_value(env::var_os("RUST_LOG"), default_filter);
    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().compact())
        .try_init()
}

/// Creates an environment-aware filter without installing a global subscriber.
#[must_use]
pub fn env_filter(default_filter: &str) -> EnvFilter {
    filter_from_value(env::var_os("RUST_LOG"), default_filter)
}

fn filter_from_value(value: Option<OsString>, default_filter: &str) -> EnvFilter {
    if let Some(value) = value {
        if let Ok(value) = value.into_string() {
            if let Ok(filter) = EnvFilter::try_new(value) {
                return filter;
            }
        }
    }
    EnvFilter::new(default_filter)
}

/// A display/debug wrapper that never reveals the wrapped value.
pub struct Redacted<T>(pub T);

impl<T> std_fmt::Debug for Redacted<T> {
    fn fmt(&self, formatter: &mut std_fmt::Formatter<'_>) -> std_fmt::Result {
        formatter.write_str("[REDACTED]")
    }
}

impl<T> std_fmt::Display for Redacted<T> {
    fn fmt(&self, formatter: &mut std_fmt::Formatter<'_>) -> std_fmt::Result {
        formatter.write_str("[REDACTED]")
    }
}

impl<T> Redacted<T> {
    /// Borrows the wrapped secret for deliberate use.
    #[must_use]
    pub fn expose(&self) -> &T {
        &self.0
    }

    /// Consumes the wrapper and returns its value.
    #[must_use]
    pub fn into_inner(self) -> T {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_filter_is_valid() {
        assert!(!env_filter("info,hutool=debug").to_string().is_empty());
        assert_eq!(
            filter_from_value(None, "warn").to_string(),
            "warn".to_owned()
        );
        assert_eq!(
            filter_from_value(Some(OsString::from("debug")), "warn").to_string(),
            "debug".to_owned()
        );
        assert_eq!(
            filter_from_value(Some(OsString::from("[invalid")), "warn").to_string(),
            "warn".to_owned()
        );
        #[cfg(unix)]
        assert_eq!(
            filter_from_value(
                Some(std::os::unix::ffi::OsStringExt::from_vec(vec![0xff])),
                "warn"
            )
            .to_string(),
            "warn".to_owned()
        );
    }

    #[test]
    fn redacted_values_do_not_leak_through_formatting() {
        let secret = Redacted("api-key");
        assert_eq!(format!("{secret}"), "[REDACTED]");
        assert_eq!(format!("{secret:?}"), "[REDACTED]");
        assert_eq!(secret.expose(), &"api-key");
        assert_eq!(secret.into_inner(), "api-key");
    }

    #[test]
    fn init_is_explicit_and_can_only_install_once() {
        assert!(init("off").is_ok());
        assert!(init("off").is_err());
    }
}
