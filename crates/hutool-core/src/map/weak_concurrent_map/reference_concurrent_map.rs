//! 对齐: JVM 弱/软引用并发 Map
//!
//! Rust 无 GC 弱引用语义；提供 `HashMap` 包装占位，语义记为 planned。

#![allow(dead_code)] // 对齐 Java ReferenceConcurrentMap，暂未接线，预留

use super::weak_concurrent_map::WeakConcurrentMap;

/// 对齐 Java: `ReferenceConcurrentMap`
pub type ReferenceConcurrentMap<K, V> = WeakConcurrentMap<K, V>;
