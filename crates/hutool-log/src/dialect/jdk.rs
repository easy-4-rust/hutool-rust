//! 对齐: `cn.hutool.log.dialect.jdk` 子包
//! 来源: hutool-log/src/main/java/cn/hutool/log/dialect/jdk/
//!
//! 中文说明: JDK Logging (`java.util.logging`) 适配层，对齐 Java `cn.hutool.log.dialect.jdk.*`。
//! Rust 端通过类型别名复用 hutool 的 `TracingLog` / `LogFactory`。

use crate::{log_factory::LogFactory, tracing_log::TracingLog};

/// 对齐 Java 类: `cn.hutool.log.dialect.jdk.JdkLog`
pub type JdkLog = TracingLog;

/// 对齐 Java 类: `cn.hutool.log.dialect.jdk.JdkLogFactory`
pub type JdkLogFactory = LogFactory;
