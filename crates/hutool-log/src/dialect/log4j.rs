//! 对齐: `cn.hutool.log.dialect.log4j` 子包
//! 来源: hutool-log/src/main/java/cn/hutool/log/dialect/log4j/
//!
//! 中文说明: Apache Log4j 1.x 适配层，对齐 Java `cn.hutool.log.dialect.log4j.*`。
//! Rust 端通过类型别名复用 hutool 的 `TracingLog` / `LogFactory`。

use crate::{log_factory::LogFactory, tracing_log::TracingLog};

/// 对齐 Java 类: `cn.hutool.log.dialect.log4j.Log4jLog`
pub type Log4jLog = TracingLog;

/// 对齐 Java 类: `cn.hutool.log.dialect.log4j.Log4jLogFactory`
pub type Log4jLogFactory = LogFactory;
