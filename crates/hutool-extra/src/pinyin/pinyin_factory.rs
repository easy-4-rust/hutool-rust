//! Pinyin helpers aligned with Hutool `PinyinUtil`, backed by the `pinyin` crate.

use pinyin::{ToPinyin, ToPinyinMulti};

use super::default_pinyin_engine::DefaultPinyinEngine;

/// Factory returning the default engine (Hutool multi-engine SPI collapsed to one Rust crate).
pub struct PinyinFactory;

impl PinyinFactory {
    /// Java: `PinyinFactory.get()` / `create()`
    #[must_use]
    pub fn get() -> DefaultPinyinEngine {
        DefaultPinyinEngine
    }

    /// Alias of [`Self::get`].
    #[must_use]
    pub fn create() -> DefaultPinyinEngine {
        Self::get()
    }
}
