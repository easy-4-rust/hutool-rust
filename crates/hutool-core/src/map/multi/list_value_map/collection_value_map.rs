//! 对齐: `cn.hutool.core.map.multi.AbsCollValueMap`
//! 来源: hutool-core/src/main/java/cn/hutool/core/map/multi/AbsCollValueMap.java

#![allow(dead_code)] // 对齐 Java CollectionValueMap，暂未接线，预留

use super::list_value_map::ListValueMap;

/// 对齐 Java: `CollectionValueMap` —— 与 ListValueMap 同构的显式别名类型。
pub type CollectionValueMap<K, V> = ListValueMap<K, V>;
