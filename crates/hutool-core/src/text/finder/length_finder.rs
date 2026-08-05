//! 对齐: `cn.hutool.core.text.finder.LengthFinder`

use crate::Result;

/// 对齐 Java: `LengthFinder#`
#[derive(Debug, Clone)]
pub struct LengthFinder {
    length: i32,
    text: String,
}

impl LengthFinder {
    /// 对齐 Java: `LengthFinder(int length)`
    pub fn new(length: i32) -> Self {
        Self {
            length,
            text: String::new(),
        }
    }

    /// 对齐 Java: `setText`
    pub fn set_text(mut self, text: &str) -> Self {
        self.text = text.to_string();
        self
    }

    /// 对齐 Java: `start`
    pub fn start(&self, from: i32) -> Result<i32> {
        let from = from.max(0);
        let end = from + self.length;
        let len = self.text.chars().count() as i32;
        if end > len { Ok(-1) } else { Ok(from) }
    }

    /// 对齐 Java: `end`
    pub fn end(&self, start: i32) -> i32 {
        if start < 0 { -1 } else { start + self.length }
    }
}
