//! 对齐: `cn.hutool.log.level.DebugLog`
//! 来源: hutool-log/src/main/java/cn/hutool/log/level/DebugLog.java
//!
//! 中文说明: Hutool Debug 级别日志接口，对齐 Java 接口 `cn.hutool.log.level.DebugLog`。

use crate::log::Log;

/// Hutool Debug 级别日志接口，对齐 Java 接口 `cn.hutool.log.level.DebugLog`。
///
/// 对齐 Java 接口: `cn.hutool.log.level.DebugLog`
/// 来源: hutool-log/src/main/java/cn/hutool/log/level/DebugLog.java
pub trait DebugLog: Log {
    /// `DEBUG` 级别是否启用，等价 Java `DebugLog.isDebugEnabled()`。
    fn is_debug_enabled(&self) -> bool;

    /// 输出 `DEBUG` 级别日志，等价 Java `DebugLog.debug(...)`。
    fn debug(&self, message: &str);

    /// 输出 `DEBUG` 级别日志（带参数），等价 Java `DebugLog.debug(format, args)`。
    fn debug_fmt(&self, template: &str, args: &[&dyn std::fmt::Display]);

    /// 输出 `DEBUG` 级别日志（Throwable），等价 Java `DebugLog.debug(Throwable)`。
    fn debug_throwable(&self, message: &str, throwable: &dyn std::error::Error);
}
