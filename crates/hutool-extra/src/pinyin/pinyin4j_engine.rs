//! Pinyin helpers aligned with Hutool `PinyinUtil`, backed by the `pinyin` crate.

use super::default_pinyin_engine::DefaultPinyinEngine;

/// Alias for [`DefaultPinyinEngine`] (Hutool `Pinyin4jEngine`).
pub type Pinyin4jEngine = DefaultPinyinEngine;
