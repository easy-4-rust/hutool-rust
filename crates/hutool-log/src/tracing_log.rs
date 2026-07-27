//! 对齐: `cn.hutool.log.Log` (具体实现)
//! 来源: hutool-log/src/main/java/cn/hutool/log/AbstractLog.java
//! 中文说明: 基于 tracing 的原生 Hutool 日志实现，各方言（Log4j/Slf4j 等）的统一别名。

use std::{
    collections::HashMap,
    fmt,
    sync::{Arc, OnceLock, RwLock},
};

use crate::abstract_log::AbstractLog;

/// 基于 tracing 的原生 Hutool 日志实现类型别名，对应 `cn.hutool.log.AbstractLog`。
///
/// 对齐 Java 类: `cn.hutool.log.AbstractLog` 的具体实现
/// 来源: hutool-log/src/main/java/cn/hutool/log/AbstractLog.java
///
/// The native `HiTool` logger; compatibility dialect names are aliases of this type.
pub type TracingLog = AbstractLog;
