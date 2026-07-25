//! 对齐: `cn.hutool.log` 兼容层模块
//! 来源: hutool-log/src/main/java/cn/hutool/log/
//! 中文说明: Hutool 日志兼容层，将 Java 日志抽象（Log/LogFactory/StaticLog 等）映射到 Rust tracing 生态。

use std::{
    collections::HashMap,
    fmt,
    sync::{Arc, OnceLock, RwLock},
};

mod log_level;
mod log_record;
mod log_sink;
mod tracing_sink;
mod log;
mod abstract_log;
mod tracing_log;
mod log_factory;
mod global_log_factory;
mod static_log;

pub use log_level::LogLevel;
pub use log_record::LogRecord;
pub use log_sink::LogSink;
pub use tracing_sink::TracingSink;
pub use log::Log;
pub use abstract_log::AbstractLog;
pub use tracing_log::TracingLog;
pub use log_factory::LogFactory;
pub use global_log_factory::GlobalLogFactory;
pub use static_log::StaticLog;

pub fn format_message(template: &str, arguments: &[&dyn fmt::Display]) -> String {
    if arguments.is_empty() {
        return template.to_owned();
    }
    let mut result = String::with_capacity(template.len());
    let mut remaining = template;
    let mut arguments = arguments.iter();
    while let Some(index) = remaining.find("{}") {
        result.push_str(&remaining[..index]);
        if let Some(argument) = arguments.next() {
            result.push_str(&argument.to_string());
        } else {
            result.push_str("{}");
            remaining = &remaining[index + 2..];
            result.push_str(remaining);
            return result;
        }
        remaining = &remaining[index + 2..];
    }
    result.push_str(remaining);
    result
}

fn global_slot() -> &'static RwLock<LogFactory> {
    static FACTORY: OnceLock<RwLock<LogFactory>> = OnceLock::new();
    FACTORY.get_or_init(|| RwLock::new(LogFactory::default()))
}
