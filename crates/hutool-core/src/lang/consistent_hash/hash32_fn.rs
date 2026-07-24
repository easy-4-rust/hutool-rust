//! 对齐: `cn.hutool.core.lang.ConsistentHash`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/ConsistentHash.java
//!
//! 一致性哈希环：`BTreeMap` 模拟 Java `TreeMap`，默认 FNV32 哈希。

use crate::hash_util::HashUtil;
use std::collections::BTreeMap;
use std::fmt::Display;
use std::hash::{Hash, Hasher};

/// 32 位哈希函数，对齐 Java `Hash32<Object>`。
pub type Hash32Fn = Box<dyn Fn(&str) -> i32 + Send + Sync>;
