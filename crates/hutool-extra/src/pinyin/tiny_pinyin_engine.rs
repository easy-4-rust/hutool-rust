//! Pinyin helpers aligned with Hutool `PinyinUtil`, backed by the `pinyin` crate.

use super::default_pinyin_engine::DefaultPinyinEngine;

/// Alias for [`DefaultPinyinEngine`] (Hutool `TinyPinyinEngine`).
pub type TinyPinyinEngine = DefaultPinyinEngine;
