//! 对齐: `cn.hutool.core.lang.ConsistentHash`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/ConsistentHash.java
//!
//! 一致性哈希环：`BTreeMap` 模拟 Java `TreeMap`，默认 FNV32 哈希。

use crate::hash_util::HashUtil;
use std::collections::BTreeMap;
use std::fmt::Display;
use std::hash::{Hash, Hasher};

mod hash32_fn;
mod consistent_hash;
mod str_node;

pub use hash32_fn::Hash32Fn;
pub use consistent_hash::ConsistentHash;
pub use str_node::StrNode;
