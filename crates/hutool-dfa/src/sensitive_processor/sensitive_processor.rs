//! 对齐: `cn.hutool.dfa.SensitiveProcessor` (Rust 独有 trait)
//! 中文说明: 敏感词处理器 trait，定义匹配到敏感词后的替换逻辑

use crate::{DfaError, FoundWord, MatchOptions, WordTree};
use parking_lot::RwLock;
use serde::{Serialize, de::DeserializeOwned};
use std::{collections::BTreeMap, sync::Arc, thread::JoinHandle};

/// Rewrites one sensitive match.
pub trait SensitiveProcessor: Send + Sync {
    /// Produces replacement text; the default emits one `*` per Unicode scalar.
    fn process(&self, found_word: &FoundWord) -> String {
        "*".repeat(found_word.found_word().chars().count())
    }
}

impl<F> SensitiveProcessor for F
where
    F: Fn(&FoundWord) -> String + Send + Sync,
{
    fn process(&self, found_word: &FoundWord) -> String {
        self(found_word)
    }
}
