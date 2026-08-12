use std::sync::{OnceLock, RwLock};

use super::serialize_registry::SerializeRegistry;

/// 对齐: `cn.hutool.json.GlobalSerializeMapping`
/// 中文说明: Hutool 全局序列化映射的兼容访问接口。
///
/// Explicit compatibility access to Hutool's global serialization mapping.
pub struct GlobalSerializeMapping;

impl GlobalSerializeMapping {
    /// 中文说明: 返回共享已注册闭包的快照。
    /// 对齐 Java 方法: `get`
    ///
    /// # Panics
    ///
    /// Panics when another thread poisoned the compatibility-global lock.
    #[must_use]
    pub fn get() -> SerializeRegistry {
        global_slot()
            .read()
            .expect("global JSON mapping read lock poisoned")
            .clone()
    }

    /// 中文说明: 替换兼容全局映射并返回之前的映射。
    /// 对齐 Java 方法: `set`
    ///
    /// # Panics
    ///
    /// Panics when another thread poisoned the compatibility-global lock.
    pub fn set(registry: SerializeRegistry) -> SerializeRegistry {
        std::mem::replace(
            &mut *global_slot()
                .write()
                .expect("global JSON mapping write lock poisoned"),
            registry,
        )
    }

    /// 中文说明: 恢复为空的全局映射。
    /// 对齐 Java 方法: `reset`
    pub fn reset() -> SerializeRegistry {
        Self::set(SerializeRegistry::new())
    }
}

fn global_slot() -> &'static RwLock<SerializeRegistry> {
    static REGISTRY: OnceLock<RwLock<SerializeRegistry>> = OnceLock::new();
    REGISTRY.get_or_init(|| RwLock::new(SerializeRegistry::new()))
}
