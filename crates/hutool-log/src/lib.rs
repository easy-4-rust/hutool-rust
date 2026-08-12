//! 对齐: `cn.hutool.log` 日志门面模块
//! 来源: hutool-log/src/main/java/cn/hutool/log/
//! 中文说明: 基于 Rust `tracing` 生态的结构化应用日志模块，提供与 Hutool Java 日志体系兼容的抽象层。
//!
//! 目录结构（按 Java 包 1:1 对齐）：
//! - 顶级包：`src/{log,abstract_log,log_factory,global_log_factory,static_log,log_record,log_sink,tracing_sink,tracing_log,format_message}.rs`  (cn.hutool.log.*)
//! - level 子包：`src/level/{level,trace_log,debug_log,info_log,warn_log,error_log}.rs` (cn.hutool.log.level.*)
//! - dialect 子包：`src/dialect/{commons,console,jboss,jdk,log4j,log4j2,logtube,slf4j,tinylog}.rs` (cn.hutool.log.dialect.*)
//!
//! Structured application logging built on the Rust `tracing` ecosystem.

#![forbid(unsafe_code)]

use std::fmt as std_fmt;
use std::{env, ffi::OsString};
pub use tracing::{Level, debug, error, event, info, instrument, span, trace, warn};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

// 顶级包：cn.hutool.log.*
mod abstract_log;
pub(crate) mod format_message;
mod global_log_factory;
mod log;
mod log_factory;
mod log_record;
mod log_sink;
pub mod prelude;
mod static_log;
mod tracing_log;
mod tracing_sink;

// 子包：cn.hutool.log.level.*
pub mod level;

// 子包：cn.hutool.log.dialect.*
pub mod dialect;

pub use abstract_log::AbstractLog;
pub use format_message::format_message;
pub use global_log_factory::GlobalLogFactory;
pub use log::Log;
pub use log_factory::LogFactory;
pub use log_record::LogRecord;
pub use log_sink::LogSink;
pub use static_log::StaticLog;
pub use tracing_log::TracingLog;
pub use tracing_sink::TracingSink;

// 兼容：原顶级 `LogLevel` 别名（外部消费者可能使用 `crate::LogLevel`）
pub use level::Level as LogLevel;

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
