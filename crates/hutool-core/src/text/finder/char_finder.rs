//! 对齐: `cn.hutool.core.text.finder.CharFinder`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/finder/CharFinder.java

use crate::Result;

/// 对齐 Java: `CharFinder#`
#[derive(Debug, Clone)]
pub struct CharFinder {
    ch: char,
    case_insensitive: bool,
    negative: bool,
    text: String,
}

impl CharFinder {
    /// 对齐 Java: `CharFinder(char c)`
    pub fn new(c: char) -> Self {
        Self::with_case(c, false)
    }

    /// 对齐 Java: `CharFinder(char c, boolean caseInsensitive)`
    pub fn with_case(c: char, case_insensitive: bool) -> Self {
        Self {
            ch: c,
            case_insensitive,
            negative: false,
            text: String::new(),
        }
    }

    /// 对齐 Java: `setText`
    pub fn set_text(mut self, text: &str) -> Self {
        self.text = text.to_string();
        self
    }

    /// 对齐 Java: `setNegative`
    pub fn set_negative(mut self, negative: bool) -> Self {
        self.negative = negative;
        self
    }

    /// 对齐 Java: `CharFinder::start#int (int)`
    pub fn start(&self, from: i32) -> Result<i32> {
        let chars: Vec<char> = self.text.chars().collect();
        let len = chars.len() as i32;
        if self.negative {
            let mut i = from.min(len - 1);
            while i >= 0 {
                if self.eq(chars[i as usize]) {
                    return Ok(i);
                }
                i -= 1;
            }
            Ok(-1)
        } else {
            let mut i = from.max(0);
            while i < len {
                if self.eq(chars[i as usize]) {
                    return Ok(i);
                }
                i += 1;
            }
            Ok(-1)
        }
    }

    fn eq(&self, c: char) -> bool {
        if self.case_insensitive {
            c.eq_ignore_ascii_case(&self.ch)
        } else {
            c == self.ch
        }
    }

    /// Finder 匹配到的结束位置（单字符 +1）。
    pub fn end(&self, start: i32) -> i32 {
        if start < 0 { -1 } else { start + 1 }
    }
}
