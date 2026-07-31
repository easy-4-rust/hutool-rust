//! 对齐: `cn.hutool.core.lang.hash.Hash64`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/hash/Hash64.java

#![allow(dead_code)] // 对齐 Java Hash64 trait，暂未接线，预留

/// 对齐 Java: `Hash64<T>`
pub trait Hash64<T: ?Sized> {
    /// 对齐 Java: `hash64(T)`
    fn hash64(&self, key: &T) -> i64;
}
