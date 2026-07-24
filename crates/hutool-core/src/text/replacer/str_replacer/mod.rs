//! 对齐: `cn.hutool.core.text.replacer.StrReplacer`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/replacer/StrReplacer.java
//!
//! 抽象替换器 trait：具体替换由 `replace_at` 提供；`replace` 提供全串扫描默认实现。

use crate::Result;

mod str_replacer;
mod boxed_str_replacer;

pub use str_replacer::StrReplacer;
pub use boxed_str_replacer::BoxedStrReplacer;
