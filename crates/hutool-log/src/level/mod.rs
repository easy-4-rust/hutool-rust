//! 对齐: `cn.hutool.log.level` 子包
//! 来源: hutool-log/src/main/java/cn/hutool/log/level/
//!
//! 中文说明: Hutool 日志级别子包模块，对齐 Java `cn.hutool.log.level.*`。
//! 子包包含 6 个文件：枚举 Level + 5 个级别接口（TraceLog/DebugLog/InfoLog/WarnLog/ErrorLog）。

pub mod debug_log;
pub mod error_log;
pub mod info_log;
pub mod log_level;
pub mod trace_log;
pub mod warn_log;

pub use debug_log::DebugLog;
pub use error_log::ErrorLog;
pub use info_log::InfoLog;
pub use log_level::{Level, LogLevel};
pub use trace_log::TraceLog;
pub use warn_log::WarnLog;
