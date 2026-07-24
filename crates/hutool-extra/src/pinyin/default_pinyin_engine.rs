//! Pinyin helpers aligned with Hutool `PinyinUtil`, backed by the `pinyin` crate.

use pinyin::{ToPinyin, ToPinyinMulti};

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
