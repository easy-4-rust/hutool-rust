//! 对齐: `cn.hutool.core.text.finder.CharMatcherFinder`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/finder/CharMatcherFinder.java
//!
//! 按字符谓词查找位置（如数字、空白等）。

use crate::Result;

use super::text_finder::TextFinder;

/// 对齐 Java: `CharMatcherFinder#`
///
/// Rust 用 `fn(char) -> bool` 代替 Hutool `Matcher<Character>`。
#[derive(Clone)]
pub struct CharMatcherFinder {
    base: TextFinder,
    matcher: fn(char) -> bool,
}

impl std::fmt::Debug for CharMatcherFinder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CharMatcherFinder")
            .field("base", &self.base)
            .finish()
    }
}

impl CharMatcherFinder {
    /// 对齐 Java: `CharMatcherFinder(Matcher matcher)`
    pub fn new(matcher: fn(char) -> bool) -> Self {
        Self {
            base: TextFinder::new(),
            matcher,
        }
    }

    /// 便捷：查找空白字符。
    pub fn whitespace() -> Self {
        Self::new(|c| c.is_whitespace())
    }

    /// 便捷：查找十进制数字。
    pub fn digit() -> Self {
        Self::new(|c| c.is_ascii_digit())
    }

    /// 对齐 Java: `setText`
    pub fn set_text(mut self, text: &str) -> Self {
        self.base = self.base.set_text(text);
        self
    }

    /// 对齐 Java: `setNegative`
    pub fn set_negative(mut self, negative: bool) -> Self {
        self.base = self.base.set_negative(negative);
        self
    }

    /// 对齐 Java: `setEndIndex`
    pub fn set_end_index(mut self, end_index: i32) -> Self {
        self.base = self.base.set_end_index(end_index);
        self
    }

    /// 对齐 Java: `CharMatcherFinder::start#int (int)`
    pub fn start(&self, from: i32) -> Result<i32> {
        let limit = self.base.valid_end_index();
        let chars: Vec<char> = self.base.text.chars().collect();
        let matcher = self.matcher;
        if self.base.negative {
            let mut i = from.min(chars.len() as i32 - 1);
            while i > limit {
                if i >= 0 && matcher(chars[i as usize]) {
                    return Ok(i);
                }
                i -= 1;
            }
        } else {
            let mut i = from.max(0);
            while i < limit {
                if matcher(chars[i as usize]) {
                    return Ok(i);
                }
                i += 1;
            }
        }
        Ok(-1)
    }

    /// 对齐 Java: `CharMatcherFinder::end#int (int)`
    pub fn end(&self, start: i32) -> i32 {
        if start < 0 { -1 } else { start + 1 }
    }
}
