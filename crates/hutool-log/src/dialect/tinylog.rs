//! 对齐: `cn.hutool.log.dialect.tinylog` 子包
//! 来源: hutool-log/src/main/java/cn/hutool/log/dialect/tinylog/
//!
//! 中文说明: tinylog 适配层，对齐 Java `cn.hutool.log.dialect.tinylog.*`。
//! Rust 端通过类型别名复用 hutool 的 `TracingLog` / `LogFactory`。

use crate::{log_factory::LogFactory, tracing_log::TracingLog};

/// 对齐 Java 类: `cn.hutool.log.dialect.tinylog.TinyLog`
pub type TinyLog = TracingLog;

/// 对齐 Java 类: `cn.hutool.log.dialect.tinylog.TinyLogFactory`
pub type TinyLogFactory = LogFactory;

/// 对齐 Java 类: `cn.hutool.log.dialect.tinylog.TinyLog2`
pub type TinyLog2 = TracingLog;

/// 对齐 Java 类: `cn.hutool.log.dialect.tinylog.TinyLog2Factory`
pub type TinyLog2Factory = LogFactory;
