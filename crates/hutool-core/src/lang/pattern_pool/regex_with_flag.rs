//! 对齐: `cn.hutool.core.lang.PatternPool`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/PatternPool.java
//!
//! 编译正则缓存；flags 对齐 Java `Pattern` 位掩码的常用子集（CASE_INSENSITIVE=2）。

use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::Mutex;
use regex::{Regex, RegexBuilder};

use crate::lang::regex_pool::RegexPool;

use super::pattern_pool::PatternPool;

/// 对齐 Java: `PatternPool.RegexWithFlag`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RegexWithFlag {
    regex: String,
    flag: i32,
}

impl RegexWithFlag {
    /// 对齐 Java: `RegexWithFlag(String, int)`
    #[must_use]
    pub fn new(regex: impl Into<String>, flag: i32) -> Self {
        Self {
            regex: regex.into(),
            flag,
        }
    }
}

use super::{FLAG_CASE_INSENSITIVE, compile, pool};
