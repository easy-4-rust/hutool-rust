//! 对齐: `cn.hutool.log.dialect.console` 子包
//! 来源: hutool-log/src/main/java/cn/hutool/log/dialect/console/
//!
//! 中文说明: 控制台日志适配层，对齐 Java `cn.hutool.log.dialect.console.*`。
//! `ConsoleLog` / `ConsoleColorLog` 都通过 `TracingLog` 类型别名复用 hutool 的 tracing 后端。

use crate::{log_factory::LogFactory, tracing_log::TracingLog};

/// 对齐 Java 类: `cn.hutool.log.dialect.console.ConsoleLog`
pub type ConsoleLog = TracingLog;

/// 对齐 Java 类: `cn.hutool.log.dialect.console.ConsoleLogFactory`
pub type ConsoleLogFactory = LogFactory;

/// 对齐 Java 类: `cn.hutool.log.dialect.console.ConsoleColorLog`
pub type ConsoleColorLog = TracingLog;

/// 对齐 Java 类: `cn.hutool.log.dialect.console.ConsoleColorLogFactory`
pub type ConsoleColorLogFactory = LogFactory;
