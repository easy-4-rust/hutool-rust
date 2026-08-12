//! 对齐: `cn.hutool.core.map.TreeEntry` / `ForestMap` / `LinkedForestMap`
//! 来源: hutool-core/.../LinkedForestMap.java（简化可运行实现）

#![allow(dead_code)] // 对齐 Java ForestMap，暂未接线，预留

use super::linked_forest_map::LinkedForestMap;

/// 对齐 Java 接口: `cn.hutool.core.map.ForestMap`
///
/// 以 `LinkedForestMap` 为默认实现。
pub type ForestMap<K, V> = LinkedForestMap<K, V>;
