//! 对齐: `cn.hutool.core.text.replacer.ReplacerChain`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/replacer/ReplacerChain.java
//!
//! 将多个查找替换器串成链，按顺序对同一文本应用。

use crate::text::replacer::lookup_replacer::LookupReplacer;

/// 可链式执行的替换步骤。
#[derive(Debug, Clone)]
pub enum ChainStep {
    /// Lookup 表替换。
    Lookup(LookupReplacer),
    /// 自定义全串函数。
    Fn(fn(&str) -> String),
}
