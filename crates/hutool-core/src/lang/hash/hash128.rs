//! 对齐: `cn.hutool.core.lang.hash.Hash128`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/hash/Hash128.java

#![allow(dead_code)] // 对齐 Java Hash128 trait，暂未接线，预留

use super::metro_hash::Number128;

/// 对齐 Java: `Hash128<T>`
pub trait Hash128<T: ?Sized> {
    /// 对齐 Java: `hash128(T)`
    fn hash128(&self, key: &T) -> Number128;
}
