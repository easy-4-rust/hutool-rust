//! 对齐: `cn.hutool.core.text.replacer.ReplacerChain`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/replacer/ReplacerChain.java
//!
//! 将多个查找替换器串成链，按顺序对同一文本应用。

use crate::Result;

use crate::text::replacer::lookup_replacer::LookupReplacer;

use super::chain_step::ChainStep;

/// 对齐 Java: `ReplacerChain#`
#[derive(Debug, Clone, Default)]
pub struct ReplacerChain {
    steps: Vec<ChainStep>,
}

impl ReplacerChain {
    /// 对齐 Java: `ReplacerChain()`
    pub fn new() -> Self {
        Self::default()
    }

    /// 对齐 Java: `ReplacerChain::addChain` / `add` — 追加 LookupReplacer。
    pub fn add(&mut self, replacer: LookupReplacer) -> Result<&mut Self> {
        self.steps.push(ChainStep::Lookup(replacer));
        Ok(self)
    }

    /// 追加自定义全串替换函数。
    pub fn add_fn(&mut self, f: fn(&str) -> String) -> Result<&mut Self> {
        self.steps.push(ChainStep::Fn(f));
        Ok(self)
    }

    /// 对齐 Java: `iterator` — 步骤数量（便于测试）。
    pub fn len(&self) -> usize {
        self.steps.len()
    }

    /// 链是否为空。
    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }

    /// 对齐 Java: `ReplacerChain::replace#String (CharSequence)`
    pub fn replace(&self, text: &str) -> Result<String> {
        let mut current = text.to_string();
        for step in &self.steps {
            current = match step {
                ChainStep::Lookup(r) => r.replace(&current)?,
                ChainStep::Fn(f) => f(&current),
            };
        }
        Ok(current)
    }

    /// 对齐 Java: `iterator` — 返回步骤切片。
    pub fn iter(&self) -> impl Iterator<Item = &ChainStep> {
        self.steps.iter()
    }
}
