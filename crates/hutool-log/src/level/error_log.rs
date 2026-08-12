//! 对齐: `cn.hutool.log.level.ErrorLog`
//! 来源: hutool-log/src/main/java/cn/hutool/log/level/ErrorLog.java
//!
//! 中文说明: Hutool Error 级别日志接口，对齐 Java 接口 `cn.hutool.log.level.ErrorLog`。

use crate::log::Log;

/// Hutool Error 级别日志接口，对齐 Java 接口 `cn.hutool.log.level.ErrorLog`。
///
/// 对齐 Java 接口: `cn.hutool.log.level.ErrorLog`
/// 来源: hutool-log/src/main/java/cn/hutool/log/level/ErrorLog.java
pub trait ErrorLog: Log {
    /// `ERROR` 级别是否启用，等价 Java `ErrorLog.isErrorEnabled()`。
    fn is_error_enabled(&self) -> bool;

    /// 输出 `ERROR` 级别日志，等价 Java `ErrorLog.error(...)`。
    fn error(&self, message: &str);

    /// 输出 `ERROR` 级别日志（带参数），等价 Java `ErrorLog.error(format, args)`。
    fn error_fmt(&self, template: &str, args: &[&dyn std::fmt::Display]);

    /// 输出 `ERROR` 级别日志（Throwable），等价 Java `ErrorLog.error(Throwable)`。
    fn error_throwable(&self, message: &str, throwable: &dyn std::error::Error);
}
