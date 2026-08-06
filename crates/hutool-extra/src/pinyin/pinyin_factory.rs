//! Pinyin helpers aligned with Hutool `PinyinUtil`, backed by the `pinyin` crate.

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pinyin::pinyin_engine::PinyinEngine;

    #[test]
    fn factory_returns_default_engine() {
        // Java PinyinFactory.get() 返回单例引擎；Rust 固定 DefaultPinyinEngine
        let engine = PinyinFactory::get();
        assert_eq!(engine.get_pinyin_char('中', false), "zhong");
        assert_eq!(
            PinyinFactory::create().get_pinyin_str("中国", " ", false),
            "zhong guo"
        );
    }
}
