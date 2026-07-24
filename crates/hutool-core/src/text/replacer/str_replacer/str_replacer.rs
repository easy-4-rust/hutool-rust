//! 对齐: `cn.hutool.core.text.replacer.StrReplacer`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/replacer/StrReplacer.java
//!
//! 抽象替换器 trait：具体替换由 `replace_at` 提供；`replace` 提供全串扫描默认实现。

use crate::Result;

/// 对齐 Java: `StrReplacer` 抽象类 → Rust trait。
pub trait StrReplacer: Send + Sync {
    /// 对齐 Java: `StrReplacer::replace#String (CharSequence)`
    fn replace(&self, text: &str) -> Result<String> {
        let chars: Vec<char> = text.chars().collect();
        let mut out = String::with_capacity(text.len());
        let mut pos = 0usize;
        while pos < chars.len() {
            match self.replace_at(&chars, pos)? {
                Some((consumed, fragment)) if consumed > 0 => {
                    out.push_str(&fragment);
                    pos += consumed;
                }
                _ => {
                    out.push(chars[pos]);
                    pos += 1;
                }
            }
        }
        Ok(out)
    }

    /// 从 `pos` 尝试替换；返回 `(消耗码点数, 替换片段)`，`None` 表示未匹配。
    fn replace_at(&self, chars: &[char], pos: usize) -> Result<Option<(usize, String)>>;
}
