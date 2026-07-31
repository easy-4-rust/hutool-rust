//! `cn.hutool.core.map.reference` 子包 —— 弱/软引用 Map（planned）

pub mod reference_concurrent_map;
pub mod soft_concurrent_map;
pub mod weak_key_concurrent_map;
pub mod weak_key_value_concurrent_map;

use crate::{CoreError, Result};

/// Soft / Weak 引用并发 map 的公共说明。
#[allow(dead_code)] // 对齐 Java Reference 并发 Map，暂未接线，预留
pub fn reference_map_status() -> Result<()> {
    Err(CoreError::PendingEngine(
        "JVM SoftReference / WeakReference concurrent maps",
    ))
}
