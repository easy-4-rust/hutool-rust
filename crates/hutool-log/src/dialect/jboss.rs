//! 对齐: `cn.hutool.log.dialect.jboss` 子包
//! 来源: hutool-log/src/main/java/cn/hutool/log/dialect/jboss/
//!
//! 中文说明: JBoss Logging 适配层，对齐 Java `cn.hutool.log.dialect.jboss.*`。
//! Rust 端通过类型别名复用 hutool 的 `TracingLog` / `LogFactory`。

use crate::{log_factory::LogFactory, tracing_log::TracingLog};

/// 对齐 Java 类: `cn.hutool.log.dialect.jboss.JbossLog`
pub type JbossLog = TracingLog;

/// 对齐 Java 类: `cn.hutool.log.dialect.jboss.JbossLogFactory`
pub type JbossLogFactory = LogFactory;
