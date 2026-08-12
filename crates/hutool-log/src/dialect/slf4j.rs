//! 对齐: `cn.hutool.log.dialect.slf4j` 子包
//! 来源: hutool-log/src/main/java/cn/hutool/log/dialect/slf4j/
//!
//! 中文说明: SLF4J 适配层，对齐 Java `cn.hutool.log.dialect.slf4j.*`。
//! Rust 端通过类型别名复用 hutool 的 `TracingLog` / `LogFactory`。

use crate::{log_factory::LogFactory, tracing_log::TracingLog};

/// 对齐 Java 类: `cn.hutool.log.dialect.slf4j.Slf4jLog`
pub type Slf4jLog = TracingLog;

/// 对齐 Java 类: `cn.hutool.log.dialect.slf4j.Slf4jLogFactory`
pub type Slf4jLogFactory = LogFactory;
