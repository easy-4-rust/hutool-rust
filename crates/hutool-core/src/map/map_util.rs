//! 对齐: `cn.hutool.core.map.MapUtil` 包路径 facade
//!
//! 委托到 crate 根 [`crate::MapUtil`]。

pub use crate::map_util::LinkedOrHashMap;

/// 历史别名：有序/无序 Map 联合体。
#[allow(dead_code)] // 对齐 Java MapUtil 历史别名，暂未接线，预留
pub type EitherMap<K, V> = LinkedOrHashMap<K, V>;
