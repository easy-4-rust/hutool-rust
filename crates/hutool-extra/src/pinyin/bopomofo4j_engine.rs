//! Pinyin helpers aligned with Hutool `PinyinUtil`, backed by the `pinyin` crate.

use pinyin::{ToPinyin, ToPinyinMulti};

use super::default_pinyin_engine::DefaultPinyinEngine;

/// Alias for [`DefaultPinyinEngine`] (Hutool `Bopomofo4jEngine` — romanized pinyin, not Bopomofo).
pub type Bopomofo4jEngine = DefaultPinyinEngine;
