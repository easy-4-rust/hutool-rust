//! 对齐: `cn.hutool.core.lang.ConsistentHash`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/ConsistentHash.java
//!
//! 一致性哈希环：`BTreeMap` 模拟 Java `TreeMap`，默认 FNV32 哈希。

use crate::hash_util::HashUtil;
use std::collections::BTreeMap;
use std::fmt::Display;
use std::hash::{Hash, Hasher};

/// 默认字符串键的 Display 包装，便于测试。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrNode(pub String);

impl Display for StrNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl Hash for StrNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
