//! 对齐: `cn.hutool.core.lang.Tuple`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/Tuple.java
//!
//! 异构元素用 `serde_json::Value` 承载，保留 Eq/Hash/Display；
//! Java Stream/Spliterator 映射为迭代器与 `to_list`。

#![allow(dead_code)] // 对齐 Java Tuple，暂未接线，预留

use serde_json::Value;
use std::hash::{Hash, Hasher};

/// 对齐 Java: `Tuple`
#[derive(Debug, Clone)]
pub struct Tuple {
    members: Vec<Value>,
    cache_hash: bool,
    cached: Option<u64>,
}

impl Tuple {
    /// 对齐 `new Tuple(Object...)`
    pub fn new(members: Vec<Value>) -> Self {
        Self {
            members,
            cache_hash: false,
            cached: None,
        }
    }

    /// 从可序列化显示值构造（字符串化）。
    pub fn from_display(values: &[impl ToString]) -> Self {
        Self::new(
            values
                .iter()
                .map(|v| Value::String(v.to_string()))
                .collect(),
        )
    }

    /// 对齐 `get(int)`
    pub fn get(&self, index: usize) -> Option<&Value> {
        self.members.get(index)
    }

    /// 对齐 `getMembers`
    #[must_use]
    pub fn get_members(&self) -> &[Value] {
        &self.members
    }

    /// 对齐 `toList`
    #[must_use]
    pub fn to_list(&self) -> Vec<Value> {
        self.members.clone()
    }

    /// 对齐 `setCacheHash`
    pub fn set_cache_hash(&mut self, cache: bool) -> &mut Self {
        self.cache_hash = cache;
        if !cache {
            self.cached = None;
        }
        self
    }

    /// 对齐 `size`
    #[must_use]
    pub fn size(&self) -> usize {
        self.members.len()
    }

    /// 对齐 `contains`
    pub fn contains(&self, value: &Value) -> bool {
        self.members.iter().any(|m| m == value)
    }

    /// 对齐 `sub(start, end)` — 半开区间 `[start, end)`
    pub fn sub(&self, start: usize, end: usize) -> Tuple {
        let end = end.min(self.members.len());
        let start = start.min(end);
        Tuple::new(self.members[start..end].to_vec())
    }

    /// 对齐 `iterator` / `stream`
    pub fn iter(&self) -> impl Iterator<Item = &Value> {
        self.members.iter()
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        self.members == other.members
    }
}
impl Eq for Tuple {}

impl Hash for Tuple {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if self.cache_hash {
            // 调用方通过 set_cache_hash 启用缓存；此处按需计算
        }
        for m in &self.members {
            // Value 无 Hash；用 Display 稳定摘要
            m.to_string().hash(state);
        }
    }
}

impl std::fmt::Display for Tuple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, m) in self.members.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{m}")?;
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod tuple_idiomatic_parity {
    use super::*;
    use serde_json::json;

    /// 对齐 Java Tuple get/size/contains/sub 可执行证据。
    #[test]
    fn tuple_get_size_contains_sub_and_display() {
        let t = Tuple::new(vec![json!(1), json!("a"), json!(true)]);
        assert_eq!(t.size(), 3);
        assert_eq!(t.get(1), Some(&json!("a")));
        assert!(t.contains(&json!(true)));
        let sub = t.sub(1, 3);
        assert_eq!(sub.size(), 2);
        assert_eq!(sub.to_list(), vec![json!("a"), json!(true)]);
        assert!(t.to_string().contains('a'));
        let mut t2 = t.clone();
        t2.set_cache_hash(true);
        assert_eq!(t, t2);
    }
}
