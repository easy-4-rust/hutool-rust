//! 对齐: `cn.hutool.core.lang.ConsistentHash`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/ConsistentHash.java
//!
//! 一致性哈希环：`BTreeMap` 模拟 Java `TreeMap`，默认 FNV32 哈希。

#![allow(dead_code)] // 对齐 Java ConsistentHash，暂未接线，预留

/// 32 位哈希函数，对齐 Java `Hash32<Object>`。
pub type Hash32Fn = Box<dyn Fn(&str) -> i32 + Send + Sync>;
