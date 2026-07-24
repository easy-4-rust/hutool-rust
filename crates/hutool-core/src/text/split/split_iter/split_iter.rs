//! 对齐: `cn.hutool.core.text.split.SplitIter`
//! 来源: hutool-core SplitIter.java

use crate::text::finder::char_finder::CharFinder;
use crate::text::finder::length_finder::LengthFinder;
use crate::text::finder::pattern_finder::PatternFinder;
use crate::text::finder::str_finder::StrFinder;
use crate::{CoreError, Result};

use super::text_finder_kind::TextFinderKind;

/// 对齐 Java: `SplitIter#`
#[derive(Debug, Clone)]
pub struct SplitIter {
    text: String,
    finder: TextFinderKind,
    limit: i32,
    ignore_empty: bool,
    offset: i32,
    count: i32,
    finished: bool,
    /// 上一刀正好切在末尾分隔符上，需要再产出一个空段
    pending_empty: bool,
}

impl SplitIter {
    /// 对齐 Java: `SplitIter(CharSequence, TextFinder, int limit, boolean ignoreEmpty)`
    pub fn new(text: &str, finder: TextFinderKind, limit: i32, ignore_empty: bool) -> Result<Self> {
        let finder = finder.set_text(text);
        Ok(Self {
            text: text.to_string(),
            finder,
            limit: if limit <= 0 { i32::MAX } else { limit },
            ignore_empty,
            offset: 0,
            count: 0,
            finished: false,
            pending_empty: false,
        })
    }

    /// 从 CharFinder 构造
    pub fn by_char(text: &str, finder: CharFinder, limit: i32, ignore_empty: bool) -> Result<Self> {
        Self::new(text, TextFinderKind::Char(finder), limit, ignore_empty)
    }

    /// 从 StrFinder 构造
    pub fn by_str(text: &str, finder: StrFinder, limit: i32, ignore_empty: bool) -> Result<Self> {
        Self::new(text, TextFinderKind::Str(finder), limit, ignore_empty)
    }

    /// 对齐 Java: `reset`
    pub fn reset(&mut self) {
        self.offset = 0;
        self.count = 0;
        self.finished = false;
        self.pending_empty = false;
    }

    /// 对齐 Java: `toList(boolean trim)`
    pub fn to_list(&mut self, trim: bool) -> Result<Vec<String>> {
        let mut out = Vec::new();
        while let Some(part) = self.next_part()? {
            let part = if trim { part.trim().to_string() } else { part };
            if self.ignore_empty && part.is_empty() {
                continue;
            }
            // when trim emptied an ignore_empty item after trim — already handled
            if trim && self.ignore_empty && part.is_empty() {
                continue;
            }
            out.push(part);
        }
        // Re-filter empty after trim when ignore_empty
        if trim && self.ignore_empty {
            out.retain(|s| !s.is_empty());
        }
        Ok(out)
    }

    fn next_part(&mut self) -> Result<Option<String>> {
        if self.finished {
            return Ok(None);
        }
        if self.pending_empty {
            self.pending_empty = false;
            self.finished = true;
            self.count += 1;
            return Ok(Some(String::new()));
        }
        let chars: Vec<char> = self.text.chars().collect();
        let len = chars.len() as i32;

        if self.text.is_empty() {
            self.finished = true;
            return Ok(Some(String::new()));
        }

        if self.count >= self.limit - 1 {
            // last segment: rest of string
            self.finished = true;
            if self.offset >= len {
                return Ok(None);
            }
            let rest: String = chars[self.offset as usize..].iter().collect();
            self.count += 1;
            return Ok(Some(rest));
        }

        let start = self.finder.start(self.offset)?;
        if start < 0 {
            self.finished = true;
            if self.offset >= len {
                return Ok(None);
            }
            let rest: String = chars[self.offset as usize..].iter().collect();
            self.count += 1;
            return Ok(Some(rest));
        }

        let end = self.finder.end(start);
        let part: String = chars[self.offset as usize..start as usize].iter().collect();
        self.offset = end;
        self.count += 1;

        // skip empty separators that don't advance (infinite loop guard)
        if end <= start {
            return Err(CoreError::InvalidArgument {
                name: "finder",
                reason: "finder end <= start",
            });
        }
        // 分隔符落在末尾 → 下次再产出空段（CharFinder）；LengthFinder 切满则 end==len 且不应空段
        if end == len {
            match &self.finder {
                TextFinderKind::Length(_) => {}
                _ => self.pending_empty = true,
            }
        }
        Ok(Some(part))
    }
}
