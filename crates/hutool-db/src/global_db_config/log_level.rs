//! GlobalDbConfig facade，对齐 hutool 的 `cn.hutool.db.GlobalDbConfig`。
//!
//! 全局数据库配置（大小写敏感、是否返回生成键、是否显示 SQL 等）。

use std::sync::Mutex;

/// 日志级别，对齐 `cn.hutool.db.nosql.NoSQLException` 等使用的 Level（简化版）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}
