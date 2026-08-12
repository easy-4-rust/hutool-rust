//! 对齐: `cn.hutool.core.lang.hash.Hash`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/hash/Hash.java

#![allow(dead_code)] // 对齐 Java Hash trait，暂未接线，预留

/// 对齐 Java: `Hash<T>` — 通用哈希，返回 `i64` 以覆盖 Number 语义。
pub trait Hash<T: ?Sized> {
    /// 对齐 Java: `hash(T)`
    fn hash(&self, key: &T) -> i64;
}
