//! Pinyin helpers aligned with Hutool `PinyinUtil`, backed by the `pinyin` crate.

use pinyin::{ToPinyin, ToPinyinMulti};

/// Engine trait mirroring Hutool `PinyinEngine`.
pub trait PinyinEngine: Send + Sync {
    /// Converts a single Chinese character.
    fn get_pinyin_char(&self, c: char, tone: bool) -> String;
    /// Converts a string with separator.
    fn get_pinyin_str(&self, str: &str, separator: &str, tone: bool) -> String;
}
