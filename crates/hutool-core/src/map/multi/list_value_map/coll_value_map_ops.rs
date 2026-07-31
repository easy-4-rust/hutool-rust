//! 对齐: `cn.hutool.core.map.multi.AbsCollValueMap`
//! 来源: hutool-core/src/main/java/cn/hutool/core/map/multi/AbsCollValueMap.java

#![allow(dead_code)] // 对齐 Java AbsCollValueMap，暂未接线，预留

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
