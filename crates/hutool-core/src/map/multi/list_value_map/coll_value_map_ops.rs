//! 对齐: `cn.hutool.core.map.multi.AbsCollValueMap`
//! 来源: hutool-core/src/main/java/cn/hutool/core/map/multi/AbsCollValueMap.java

use std::collections::HashMap;
use std::hash::Hash;

/// 值集合 Map 的公共行为 —— 对齐 `AbsCollValueMap`。
pub trait CollValueMapOps<K, V> {
    /// 对齐 Java: `putValue`
    fn put_value(&mut self, key: K, value: V);
    /// 对齐 Java: `getValues` / get collection
    fn get_values(&self, key: &K) -> Option<&[V]>;
    /// 对齐 Java: `removeValue`
    fn remove_value(&mut self, key: &K, value: &V) -> bool
    where
        V: PartialEq;
}
