//! 对齐: `cn.hutool.core.lang.Pair`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/Pair.java

#![allow(dead_code)] // 对齐 Java Pair，暂未接线，预留

use std::fmt;
use std::hash::{Hash, Hasher};

/// 对齐 Java: `cn.hutool.core.lang.Pair`
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pair<K, V> {
    key: K,
    value: V,
}

impl<K, V> Pair<K, V> {
    /// 对齐 Java: `Pair.of(K, V)` / `Pair(K, V)`
    #[must_use]
    pub fn of(key: K, value: V) -> Self {
        Self { key, value }
    }

    /// 对齐 Java: `Pair.getKey()`
    pub fn get_key(&self) -> &K {
        &self.key
    }

    /// 对齐 Java: `Pair.getValue()`
    pub fn get_value(&self) -> &V {
        &self.value
    }

    /// 消费并拆成元组。
    pub fn into_parts(self) -> (K, V) {
        (self.key, self.value)
    }
}

impl<K: Hash, V: Hash> Hash for Pair<K, V> {
    /// 对齐 Java: `Objects.hashCode(key) ^ Objects.hashCode(value)`
    fn hash<H: Hasher>(&self, state: &mut H) {
        // 使用标准哈希再异或，贴近 Java HashMap.Node 风格。
        let mut hk = std::collections::hash_map::DefaultHasher::new();
        self.key.hash(&mut hk);
        let mut hv = std::collections::hash_map::DefaultHasher::new();
        self.value.hash(&mut hv);
        state.write_u64(hk.finish() ^ hv.finish());
    }
}

impl<K: fmt::Display, V: fmt::Display> fmt::Display for Pair<K, V> {
    /// 对齐 Java: `Pair.toString()`
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Pair [key={}, value={}]", self.key, self.value)
    }
}

/// 对齐 Java: `Pair.equals(Object)`
impl<K: PartialEq, V: PartialEq> Pair<K, V> {
    /// 显式 equals，与 `PartialEq` 一致。
    pub fn equals(&self, other: &Self) -> bool {
        self == other
    }
}

#[cfg(test)]
mod pair_idiomatic_parity {
    use super::*;

    /// 对齐 Java Pair 构造/取值/相等的可执行证据。
    #[test]
    fn pair_of_getters_equals_and_display() {
        let p = Pair::of("k", 1);
        assert_eq!(*p.get_key(), "k");
        assert_eq!(*p.get_value(), 1);
        assert!(p.equals(&Pair::of("k", 1)));
        assert_eq!(p.to_string(), "Pair [key=k, value=1]");
    }
}
