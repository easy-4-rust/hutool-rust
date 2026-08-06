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

    /// Java: `PinyinUtil.isChinese(char)` — `〇` special-case + `[\u4e00-\u9fa5]` regex.
    #[must_use]
    pub fn is_chinese(c: char) -> bool {
        c == '〇' || matches!(c as u32, 0x4E00..=0x9FA5)
    }

    /// Java: `PinyinUtil.getPinyin(char)` / `(char, boolean tone)`
    ///
    /// 非中文返回原字符（对齐 Java 各引擎：`TinyPinyinEngine.getPinyin(char)`
    /// 对非中文返回 `String.valueOf(c)`）。
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

    /// Java: `PinyinUtil.getPinyin(String, String, boolean)`
    ///
    /// 逐字符处理并对齐 `TinyPinyinEngine` 语义：中文转拼音，非中文（含空白、标点、
    /// 数字）原样保留，每个字符之间以 `separator` 连接（对齐
    /// `TinyPinyinEngine.getPinyin(String, String)` 的 `Pinyin.toPinyin(str, separator)`）。
    #[must_use]
    pub fn get_pinyin(str: &str, separator: &str, tone: bool) -> String {
        let mut parts = Vec::with_capacity(str.chars().count());
        for ch in str.chars() {
            if Self::is_chinese(ch) {
                parts.push(Self::get_pinyin_char(ch, tone));
            } else {
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
    ///
    /// 对齐 `PinyinEngine` 默认实现 `getPinyin(c).charAt(0)`（不做小写转换）。
    #[must_use]
    pub fn get_first_letter_char(c: char) -> char {
        Self::get_pinyin_char(c, false).chars().next().unwrap_or(c)
    }

    /// Java: `PinyinUtil.getFirstLetter(String, String separator)`
    ///
    /// 对齐 `PinyinEngine` 默认实现：
    /// 1. 分隔符为空时使用 `"#"` 作为内部分隔符；
    /// 2. 对整串调用 `getPinyin(str, splitSeparator)` 并整体小写
    ///    （对齐 `TinyPinyinEngine` 的 `toLowerCase()`）；
    /// 3. 按 `splitSeparator` 分割，取每个词的首字符（空词取空串）；
    /// 4. 以 `separator` 连接。
    #[must_use]
    pub fn get_first_letter(str: &str, separator: &str) -> String {
        let split_separator = if separator.is_empty() { "#" } else { separator };
        let pinyin = Self::get_pinyin(str, split_separator, false).to_lowercase();
        let initials: Vec<String> = pinyin
            .split(split_separator)
            .map(|word| {
                word.chars()
                    .next()
                    .map(|c| c.to_string())
                    .unwrap_or_default()
            })
            .collect();
        initials.join(separator)
    }

    /// Returns all candidate pinyin readings for a character (engine multi-sound helper).
    #[must_use]
    pub fn get_pinyin_multi(c: char) -> Vec<String> {
        c.to_pinyin_multi()
            .map(|multi| multi.into_iter().map(|py| py.plain().to_string()).collect())
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_pinyin_joins_chinese_with_separator() {
        assert_eq!(PinyinUtil::get_pinyin("你好怡", " ", false), "ni hao yi");
        // 无音调 plain
        assert_eq!(PinyinUtil::get_pinyin("中国", "-", false), "zhong-guo");
    }

    #[test]
    fn get_pinyin_keeps_non_chinese_characters() {
        // 对齐 TinyPinyinEngine：非中文原样保留并参与分隔
        assert_eq!(PinyinUtil::get_pinyin("H", "#", false), "H");
        assert_eq!(
            PinyinUtil::get_pinyin("你好 hello", "#", false),
            "ni#hao# #h#e#l#l#o"
        );
        assert_eq!(PinyinUtil::get_pinyin("2026年", " ", false), "2 0 2 6 nian");
    }

    #[test]
    fn is_chinese_matches_java_regex() {
        assert!(PinyinUtil::is_chinese('你'));
        assert!(PinyinUtil::is_chinese('〇'));
        assert!(!PinyinUtil::is_chinese('H'));
        assert!(!PinyinUtil::is_chinese('1'));
        assert!(!PinyinUtil::is_chinese(' '));
    }

    #[test]
    fn get_first_letter_char_keeps_case() {
        // 对齐 PinyinEngine 默认实现 getPinyin(c).charAt(0)：不转小写
        assert_eq!(PinyinUtil::get_first_letter_char('H'), 'H');
        assert_eq!(PinyinUtil::get_first_letter_char('你'), 'n');
        assert_eq!(PinyinUtil::get_first_letter_char('好'), 'h');
    }

    #[test]
    fn get_first_letter_matches_java_tests() {
        // Java PinyinUtilTest.getFirstLetterTest()
        assert_eq!(
            PinyinUtil::get_first_letter("H是第一个", ", "),
            "h, s, d, y, g"
        );
        // Java PinyinUtilTest.getFirstLetterTest2()
        assert_eq!(PinyinUtil::get_first_letter("崞阳", ", "), "g, y");
        // Java PinyinUtilTest.getFirstLetterTest3() — null 输入在 Rust 以空串等价
        assert_eq!(PinyinUtil::get_first_letter("", ", "), "");
        // 空分隔符使用内部 "#"
        assert_eq!(PinyinUtil::get_first_letter("你好", ""), "nh");
    }

    #[test]
    fn get_pinyin_multi_lists_candidates() {
        // '长' 是多音字（chang/zhang）
        let candidates = PinyinUtil::get_pinyin_multi('长');
        assert!(!candidates.is_empty());
        assert!(candidates.iter().any(|py| py == "chang"));
        // 非中文无候选
        assert!(PinyinUtil::get_pinyin_multi('A').is_empty());
    }
}
