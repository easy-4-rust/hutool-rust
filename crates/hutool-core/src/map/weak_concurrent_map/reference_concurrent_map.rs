//! 对齐: JVM 弱/软引用并发 Map
//!
//! Rust 无 GC 弱引用语义；提供 `HashMap` 包装占位，语义记为 planned。

use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, Mutex};

use crate::{CoreError, Result};

use super::weak_concurrent_map::WeakConcurrentMap;

/// 对齐 Java: `ReferenceConcurrentMap`
pub type ReferenceConcurrentMap<K, V> = WeakConcurrentMap<K, V>;
