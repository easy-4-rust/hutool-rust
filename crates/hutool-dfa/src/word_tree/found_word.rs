//! Mutable Hutool-compatible word trie.

use crate::StopChar;
use std::{collections::HashMap, fmt, sync::Arc};

/// 单个匹配结果（UTF-8 字节偏移）。
///
/// 对齐 Java 类: `cn.hutool.dfa.FoundWord`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundWord {
    word: String,
    pub(crate) source_match: String,
    start: usize,
    end: usize,
}

impl FoundWord {
    /// Creates a match with an exclusive UTF-8 byte end offset.
    pub fn new(
        word: impl Into<String>,
        found_word: impl Into<String>,
        start: usize,
        end: usize,
    ) -> Self {
        Self {
            word: word.into(),
            source_match: found_word.into(),
            start,
            end,
        }
    }

    /// Returns the effective trie word with ignored characters removed.
    #[must_use]
    pub fn word(&self) -> &str {
        &self.word
    }

    /// Hutool-compatible alias for [`Self::word`].
    #[must_use]
    pub fn get_word(&self) -> &str {
        self.word()
    }

    /// Returns the exact substring found in the source text.
    #[must_use]
    pub fn found_word(&self) -> &str {
        &self.source_match
    }

    /// Hutool-compatible alias for [`Self::found_word`].
    #[must_use]
    pub fn get_found_word(&self) -> &str {
        self.found_word()
    }

    /// Returns the inclusive UTF-8 byte start offset.
    #[must_use]
    pub const fn start(&self) -> usize {
        self.start
    }

    /// Returns the exclusive UTF-8 byte end offset.
    #[must_use]
    pub const fn end(&self) -> usize {
        self.end
    }
}

impl fmt::Display for FoundWord {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.source_match)
    }
}

use super::{CharFilter, Node};
