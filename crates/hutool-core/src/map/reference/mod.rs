//! `cn.hutool.core.map.reference` 子包 —— 弱/软引用 Map（planned）

pub mod reference_concurrent_map;
pub mod soft_concurrent_map;
pub mod weak_key_concurrent_map;
pub mod weak_key_value_concurrent_map;

pub use weak_key_value_concurrent_map::WeakKeyValueConcurrentMap;

use crate::{CoreError, Result};

/// Soft / Weak 引用并发 map 的公共说明。
pub fn reference_map_status() -> Result<()> {
    Err(CoreError::PendingEngine(
        "JVM SoftReference / WeakReference concurrent maps",
    ))
}
