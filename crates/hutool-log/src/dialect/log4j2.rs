//! 对齐: `cn.hutool.log.dialect.log4j2` 子包
//! 来源: hutool-log/src/main/java/cn/hutool/log/dialect/log4j2/
//!
//! 中文说明: Apache Log4j 2.x 适配层，对齐 Java `cn.hutool.log.dialect.log4j2.*`。
//! Rust 端通过类型别名复用 hutool 的 `TracingLog` / `LogFactory`。

use crate::{log_factory::LogFactory, tracing_log::TracingLog};

/// 对齐 Java 类: `cn.hutool.log.dialect.log4j2.Log4j2Log`
pub type Log4j2Log = TracingLog;

/// 对齐 Java 类: `cn.hutool.log.dialect.log4j2.Log4j2LogFactory`
pub type Log4j2LogFactory = LogFactory;
