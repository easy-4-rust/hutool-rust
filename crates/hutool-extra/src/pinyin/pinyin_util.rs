//! Pinyin helpers aligned with Hutool `PinyinUtil`, backed by the `pinyin` crate.

use pinyin::{ToPinyin, ToPinyinMulti};

use super::default_pinyin_engine::DefaultPinyinEngine;
use super::pinyin_factory::PinyinFactory;

/// Hutool `PinyinUtil` facade.
///
/// Java: `cn.hutool.extra.pinyin.PinyinUtil`
pub struct PinyinUtil;

impl PinyinUtil {
    /// Java: `PinyinUtil.getEngine()`
    #[must_use]
    pub fn get_engine() -> DefaultPinyinEngine {
        PinyinFactory::get()
    }

    /// Java: `PinyinUtil.isChinese(char)` — CJK unified + Hutool `〇` special-case.
    #[must_use]
    pub fn is_chinese(c: char) -> bool {
        c == '〇' || matches!(c as u32, 0x4E00..=0x9FA5 | 0x3400..=0x4DBF)
    }

    /// Java: `PinyinUtil.getPinyin(char)` / `(char, boolean tone)`
    #[must_use]
    pub fn get_pinyin_char(c: char, tone: bool) -> String {
        let Some(py) = c.to_pinyin() else {
            return c.to_string();
        };
        if tone {
            py.with_tone().to_string()
        } else {
            py.plain().to_string()
        }
    }

    /// Java: `PinyinUtil.getPinyin(String)` / with tone / separator overloads.
    #[must_use]
    pub fn get_pinyin(str: &str, separator: &str, tone: bool) -> String {
        let mut parts = Vec::new();
        for ch in str.chars() {
            if Self::is_chinese(ch) {
                parts.push(Self::get_pinyin_char(ch, tone));
            } else if !ch.is_whitespace() {
                parts.push(ch.to_string());
            }
        }
        parts.join(separator)
    }

    /// Convenience: default separator `" "` without tone.
    #[must_use]
    pub fn get_pinyin_default(str: &str) -> String {
        Self::get_pinyin(str, " ", false)
    }

    /// Java: `PinyinUtil.getFirstLetter(char)`
    #[must_use]
    pub fn get_first_letter_char(c: char) -> char {
        Self::get_pinyin_char(c, false)
            .chars()
            .next()
            .unwrap_or(c)
            .to_ascii_lowercase()
    }

    /// Java: `PinyinUtil.getFirstLetter(String, String separator)`
    #[must_use]
    pub fn get_first_letter(str: &str, separator: &str) -> String {
        let mut parts = Vec::new();
        for ch in str.chars() {
            if Self::is_chinese(ch) {
                parts.push(Self::get_first_letter_char(ch).to_string());
            } else if ch.is_ascii_alphabetic() {
                parts.push(ch.to_ascii_lowercase().to_string());
            }
        }
        parts.join(separator)
    }

    /// Returns all candidate pinyin readings for a character (engine multi-sound helper).
    #[must_use]
    pub fn get_pinyin_multi(c: char) -> Vec<String> {
        c.to_pinyin_multi()
            .map(|multi| {
                multi
                    .into_iter()
                    .map(|py| py.plain().to_string())
                    .collect()
            })
            .unwrap_or_default()
    }
}
