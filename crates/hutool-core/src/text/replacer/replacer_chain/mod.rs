//! 对齐: `cn.hutool.core.text.replacer.ReplacerChain`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/replacer/ReplacerChain.java
//!
//! 将多个查找替换器串成链，按顺序对同一文本应用。

use crate::Result;

use crate::text::replacer::lookup_replacer::LookupReplacer;

mod chain_step;
mod replacer_chain;

pub use chain_step::ChainStep;
pub use replacer_chain::ReplacerChain;
