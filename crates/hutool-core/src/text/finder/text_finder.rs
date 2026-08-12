//! 对齐: `cn.hutool.core.text.finder.TextFinder`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/finder/TextFinder.java
//!
//! 文本查找器公共状态（被查找文本、反向、起止边界）。

use crate::Result;

/// 对齐 Java: `TextFinder#`
#[derive(Debug, Clone, Default)]
pub struct TextFinder {
    /// 被查找文本
    pub text: String,
    /// 是否反向查找
    pub negative: bool,
    /// 正向结束边界（不含）；`None` 表示到文本末尾
    pub end_index: Option<i32>,
    /// 反向结束边界（含）；`None` 表示到 0
    pub start_index: Option<i32>,
}

impl TextFinder {
    /// 创建空查找器。
    pub fn new() -> Self {
        Self::default()
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

    /// 对齐 Java: `setEndIndex` / 正向扫描上界。
    pub fn set_end_index(mut self, end: i32) -> Self {
        self.end_index = Some(end);
        self
    }

    /// 有效结束索引（正向）。
    pub fn valid_end_index(&self) -> i32 {
        let len = self.text.chars().count() as i32;
        self.end_index.unwrap_or(len).min(len).max(0)
    }

    /// 有效起始索引（反向下界）。
    pub fn valid_start_index(&self) -> i32 {
        self.start_index.unwrap_or(-1)
    }

    /// 对齐 Java: `TextFinder::start#int (int)` —— 基类无具体匹配逻辑，返回 -1。
    pub fn start(&self, _from: i32) -> Result<i32> {
        Ok(-1)
    }

    /// 对齐 Java: `TextFinder::end#int (int)`
    pub fn end(&self, start: i32) -> Result<i32> {
        if start < 0 { Ok(-1) } else { Ok(start + 1) }
    }
}
