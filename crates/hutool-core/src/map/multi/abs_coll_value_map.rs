//! 对齐: `cn.hutool.core.map.multi.AbsCollValueMap` 模块入口
//! 实际实现见 `list_value_map.rs` 中的 `ListValueMap` / `SetValueMap`。

pub use super::list_value_map::{CollectionValueMap, ListValueMap, SetValueMap};

/// 对齐 Java: `AbsCollValueMap` —— Rust 以 `ListValueMap` 为默认集合值 Map。
#[allow(dead_code)] // 对齐 Java AbsCollValueMap 别名，暂未接线，预留
pub type AbsCollValueMap<K, V> = ListValueMap<K, V>;
