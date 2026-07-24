//! 对齐: `cn.hutool.core.map.TreeEntry` / `ForestMap` / `LinkedForestMap`
//! 来源: hutool-core/.../LinkedForestMap.java（简化可运行实现）

use std::collections::HashMap;
use std::hash::Hash;

use super::linked_forest_map::LinkedForestMap;

/// 对齐 Java 接口: `cn.hutool.core.map.ForestMap`
///
/// 以 `LinkedForestMap` 为默认实现。
pub type ForestMap<K, V> = LinkedForestMap<K, V>;
