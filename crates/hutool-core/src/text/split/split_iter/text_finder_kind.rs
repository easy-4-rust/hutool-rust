//! 对齐: `cn.hutool.core.text.split.SplitIter`
//! 来源: hutool-core SplitIter.java

use crate::text::finder::char_finder::CharFinder;
use crate::text::finder::length_finder::LengthFinder;
use crate::text::finder::pattern_finder::PatternFinder;
use crate::text::finder::str_finder::StrFinder;
use crate::{CoreError, Result};

/// 查找器抽象（对齐 TextFinder）
#[derive(Debug, Clone)]
pub enum TextFinderKind {
    Char(CharFinder),
    Str(StrFinder),
    Length(LengthFinder),
    Pattern(PatternFinder),
}

impl TextFinderKind {
    pub(crate) fn set_text(self, text: &str) -> Self {
        match self {
            Self::Char(f) => Self::Char(f.set_text(text)),
            Self::Str(f) => Self::Str(f.set_text(text)),
            Self::Length(f) => Self::Length(f.set_text(text)),
            Self::Pattern(f) => Self::Pattern(f.set_text(text)),
        }
    }

    pub(crate) fn start(&self, from: i32) -> Result<i32> {
        match self {
            Self::Char(f) => f.start(from),
            Self::Str(f) => f.start(from),
            Self::Length(f) => f.start(from),
            Self::Pattern(f) => f.start(from),
        }
    }

    pub(crate) fn end(&self, start: i32) -> i32 {
        match self {
            Self::Char(f) => f.end(start),
            Self::Str(f) => f.end(start),
            Self::Length(f) => f.end(start),
            Self::Pattern(f) => f.end_of(start),
        }
    }
}
