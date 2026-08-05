//! 对齐: `cn.hutool.core.lang.PatternPool`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/PatternPool.java
//!
//! 编译正则缓存；flags 对齐 Java `Pattern` 位掩码的常用子集（CASE_INSENSITIVE=2）。

use std::sync::Arc;

use regex::Regex;

use crate::lang::regex_pool::RegexPool;

use super::regex_with_flag::RegexWithFlag;

/// 对齐 Java: `cn.hutool.core.lang.PatternPool`
pub struct PatternPool;

#[allow(dead_code)] // 对齐 Java PatternPool，部分方法暂未接线，预留
impl PatternPool {
    /// 对齐 Java: `PatternPool.GENERAL` 等预编译访问（按需编译入池）。
    pub fn general() -> Arc<Regex> {
        Self::get(RegexPool::GENERAL)
    }

    /// 对齐 Java: `PatternPool.get(String)`
    pub fn get(regex: &str) -> Arc<Regex> {
        Self::get_with_flags(regex, 0)
    }

    /// 对齐 Java: `PatternPool.get(String, int)`
    pub fn get_with_flags(regex: &str, flags: i32) -> Arc<Regex> {
        let key = RegexWithFlag::new(regex, flags);
        let mut g = pool().lock();
        if let Some(existing) = g.get(&key) {
            return Arc::clone(existing);
        }
        let compiled =
            Arc::new(compile(regex, flags).unwrap_or_else(|_| Regex::new(r"(?!)").expect("never")));
        g.insert(key, Arc::clone(&compiled));
        compiled
    }

    /// 对齐 Java: `PatternPool.remove(String, int)`
    pub fn remove(regex: &str, flags: i32) -> Option<Arc<Regex>> {
        pool().lock().remove(&RegexWithFlag::new(regex, flags))
    }

    /// 对齐 Java: `PatternPool.clear()`
    pub fn clear() {
        pool().lock().clear();
    }
}

use super::{compile, pool};
