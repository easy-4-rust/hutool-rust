//! 对齐: `cn.hutool.log.level.TraceLog`
//! 来源: hutool-log/src/main/java/cn/hutool/log/level/TraceLog.java
//!
//! 中文说明: Hutool Trace 级别日志接口，对齐 Java 接口 `cn.hutool.log.level.TraceLog`。
//! Rust 端通过扩展 `Log` trait 的 `trace` / `trace_fmt` 方法提供等价能力。
//!
//! Hutool Trace-level logging interface aligned with `cn.hutool.log.level.TraceLog`.

use crate::log::Log;

/// Hutool Trace 级别日志接口，对齐 Java 接口 `cn.hutool.log.level.TraceLog`。
///
/// 对齐 Java 接口: `cn.hutool.log.level.TraceLog`
/// 来源: hutool-log/src/main/java/cn/hutool/log/level/TraceLog.java
pub trait TraceLog: Log {
    /// `TRACE` 级别是否启用，等价 Java `TraceLog.isTraceEnabled()`。
    fn is_trace_enabled(&self) -> bool;

    /// 输出 `TRACE` 级别日志，等价 Java `TraceLog.trace(...)`。
    fn trace(&self, message: &str);

    /// 输出 `TRACE` 级别日志（带参数），等价 Java `TraceLog.trace(format, args)`。
    fn trace_fmt(&self, template: &str, args: &[&dyn std::fmt::Display]);

    /// 输出 `TRACE` 级别日志（Throwable），等价 Java `TraceLog.trace(Throwable)`。
    fn trace_throwable(&self, message: &str, throwable: &dyn std::error::Error);
}
