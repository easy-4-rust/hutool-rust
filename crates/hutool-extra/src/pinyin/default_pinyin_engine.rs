//! Pinyin helpers aligned with Hutool `PinyinUtil`, backed by the `pinyin` crate.

use super::pinyin_engine::PinyinEngine;
use super::pinyin_util::PinyinUtil;

/// Default engine using the `pinyin` crate (covers Hutool engine variants).
#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultPinyinEngine;

impl PinyinEngine for DefaultPinyinEngine {
    fn get_pinyin_char(&self, c: char, tone: bool) -> String {
        PinyinUtil::get_pinyin_char(c, tone)
    }

    fn get_pinyin_str(&self, str: &str, separator: &str, tone: bool) -> String {
        PinyinUtil::get_pinyin(str, separator, tone)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pinyin::pinyin_engine::PinyinEngine;

    #[test]
    fn default_engine_trait_methods() {
        let engine = DefaultPinyinEngine;
        // PinyinEngine::getPinyinChar
        assert_eq!(engine.get_pinyin_char('你', false), "ni");
        assert_eq!(engine.get_pinyin_char('你', true), "nǐ");
        // 非中文返回原字符
        assert_eq!(engine.get_pinyin_char('A', false), "A");
        // PinyinEngine::getPinyinStr
        assert_eq!(engine.get_pinyin_str("你好", " ", false), "ni hao");
    }
}
