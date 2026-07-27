//! 对齐: `cn.hutool.log.dialect.logtube` 子包
//! 来源: hutool-log/src/main/java/cn/hutool/log/dialect/logtube/
//!
//! 中文说明: LogTube 适配层，对齐 Java `cn.hutool.log.dialect.logtube.*`。
//! Rust 端通过类型别名复用 hutool 的 `TracingLog` / `LogFactory`。

use crate::{log_factory::LogFactory, tracing_log::TracingLog};

/// 对齐 Java 类: `cn.hutool.log.dialect.logtube.LogTubeLog`
pub type LogTubeLog = TracingLog;

/// 对齐 Java 类: `cn.hutool.log.dialect.logtube.LogTubeLogFactory`
pub type LogTubeLogFactory = LogFactory;
