//! 对齐: `cn.hutool.log.dialect.commons` 子包
//! 来源: hutool-log/src/main/java/cn/hutool/log/dialect/commons/
//!
//! 中文说明: Apache Commons Logging 适配层，对齐 Java `cn.hutool.log.dialect.commons.*`。
//! 该子包在 Rust 端通过类型别名复用 hutool 的 `TracingLog` / `LogFactory`，
//! 行为等价于 Java 端的 Apache Commons Log / Log4J Log + Factory。

use crate::{log_factory::LogFactory, tracing_log::TracingLog};

/// 对齐 Java 类: `cn.hutool.log.dialect.commons.ApacheCommonsLog`
pub type ApacheCommonsLog = TracingLog;

/// 对齐 Java 类: `cn.hutool.log.dialect.commons.ApacheCommonsLog4JLog`
pub type ApacheCommonsLog4JLog = TracingLog;

/// 对齐 Java 类: `cn.hutool.log.dialect.commons.ApacheCommonsLogFactory`
pub type ApacheCommonsLogFactory = LogFactory;
