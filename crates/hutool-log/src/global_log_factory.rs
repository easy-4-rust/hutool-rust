//! 对齐: `cn.hutool.log.StaticLogFactory`
//! 来源: hutool-log/src/main/java/cn/hutool/log/StaticLogFactory.java
//! 中文说明: 进程级全局日志工厂的兼容访问点，提供 get/set/reset 操作。

use std::sync::{OnceLock, RwLock};

use crate::log_factory::LogFactory;

/// 进程级全局日志工厂的兼容访问点，对应 Hutool 的 `StaticLogFactory`。
///
/// 对齐 Java 类: `cn.hutool.log.StaticLogFactory`
/// 来源: hutool-log/src/main/java/cn/hutool/log/StaticLogFactory.java
///
/// Explicit compatibility access to Hutool's process-wide factory.
pub struct GlobalLogFactory;

impl GlobalLogFactory {
    /// Returns a handle sharing the current global factory's cache.
    ///
    /// # Panics
    ///
    /// Panics when another thread poisoned the global factory lock.
    #[must_use]
    pub fn get() -> LogFactory {
        global_slot()
            .read()
            .expect("global log factory read lock poisoned")
            .clone()
    }
    /// Replaces the compatibility global and returns its previous value.
    ///
    /// # Panics
    ///
    /// Panics when another thread poisoned the global factory lock.
    pub fn set(factory: LogFactory) -> LogFactory {
        std::mem::replace(
            &mut *global_slot()
                .write()
                .expect("global log factory write lock poisoned"),
            factory,
        )
    }
    /// Restores the production tracing factory.
    pub fn reset() -> LogFactory {
        Self::set(LogFactory::default())
    }
}

/// 进程级 `LogFactory` 单例槽。
///
/// 对应 Java `cn.hutool.log.StaticLogFactory` 静态字段。
pub(crate) fn global_slot() -> &'static RwLock<LogFactory> {
    static FACTORY: OnceLock<RwLock<LogFactory>> = OnceLock::new();
    FACTORY.get_or_init(|| RwLock::new(LogFactory::default()))
}
