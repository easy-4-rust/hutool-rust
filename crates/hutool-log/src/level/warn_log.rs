//! 对齐: `cn.hutool.log.level.WarnLog`
//! 来源: hutool-log/src/main/java/cn/hutool/log/level/WarnLog.java
//!
//! 中文说明: Hutool Warn 级别日志接口，对齐 Java 接口 `cn.hutool.log.level.WarnLog`。

use crate::log::Log;

/// Hutool Warn 级别日志接口，对齐 Java 接口 `cn.hutool.log.level.WarnLog`。
///
/// 对齐 Java 接口: `cn.hutool.log.level.WarnLog`
/// 来源: hutool-log/src/main/java/cn/hutool/log/level/WarnLog.java
pub trait WarnLog: Log {
    /// `WARN` 级别是否启用，等价 Java `WarnLog.isWarnEnabled()`。
    fn is_warn_enabled(&self) -> bool;

    /// 输出 `WARN` 级别日志，等价 Java `WarnLog.warn(...)`。
    fn warn(&self, message: &str);

    /// 输出 `WARN` 级别日志（带参数），等价 Java `WarnLog.warn(format, args)`。
    fn warn_fmt(&self, template: &str, args: &[&dyn std::fmt::Display]);

    /// 输出 `WARN` 级别日志（Throwable），等价 Java `WarnLog.warn(Throwable)`。
    fn warn_throwable(&self, message: &str, throwable: &dyn std::error::Error);
}
