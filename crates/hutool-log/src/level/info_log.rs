//! 对齐: `cn.hutool.log.level.InfoLog`
//! 来源: hutool-log/src/main/java/cn/hutool/log/level/InfoLog.java
//!
//! 中文说明: Hutool Info 级别日志接口，对齐 Java 接口 `cn.hutool.log.level.InfoLog`。

use crate::log::Log;

/// Hutool Info 级别日志接口，对齐 Java 接口 `cn.hutool.log.level.InfoLog`。
///
/// 对齐 Java 接口: `cn.hutool.log.level.InfoLog`
/// 来源: hutool-log/src/main/java/cn/hutool/log/level/InfoLog.java
pub trait InfoLog: Log {
    /// `INFO` 级别是否启用，等价 Java `InfoLog.isInfoEnabled()`。
    fn is_info_enabled(&self) -> bool;

    /// 输出 `INFO` 级别日志，等价 Java `InfoLog.info(...)`。
    fn info(&self, message: &str);

    /// 输出 `INFO` 级别日志（带参数），等价 Java `InfoLog.info(format, args)`。
    fn info_fmt(&self, template: &str, args: &[&dyn std::fmt::Display]);

    /// 输出 `INFO` 级别日志（Throwable），等价 Java `InfoLog.info(Throwable)`。
    fn info_throwable(&self, message: &str, throwable: &dyn std::error::Error);
}
