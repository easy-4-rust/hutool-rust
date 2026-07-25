use std::any::{Any, TypeId};

use unicode_general_category::{GeneralCategory, get_general_category};

use super::char_error::CharError;

/// Hutool-compatible ASCII, Unicode-category, and enclosed-character helpers.

/// 对齐: `cn.hutool.core.util.CharUtil`
/// 字符工具类
pub struct CharUtil;

impl CharUtil {
    /// Returns whether `ch` is in the seven-bit ASCII range.
    #[must_use]
    pub const fn is_ascii(ch: char) -> bool {
        (ch as u32) < 128
    }

    /// Returns whether `ch` is a printable ASCII character.
    #[must_use]
    pub const fn is_ascii_printable(ch: char) -> bool {
        matches!(ch as u32, 32..=126)
    }

    /// Returns whether `ch` is an ASCII control character.
    #[must_use]
    pub const fn is_ascii_control(ch: char) -> bool {
        matches!(ch as u32, 0..=31 | 127)
    }

    /// Returns whether `ch` is an ASCII letter.
    #[must_use]
    pub const fn is_letter(ch: char) -> bool {
        Self::is_letter_upper(ch) || Self::is_letter_lower(ch)
    }

    /// Returns whether `ch` is an uppercase ASCII letter.
    #[must_use]
    pub const fn is_letter_upper(ch: char) -> bool {
        ch.is_ascii_uppercase()
    }

    /// Returns whether `ch` is a lowercase ASCII letter.
    #[must_use]
    pub const fn is_letter_lower(ch: char) -> bool {
        ch.is_ascii_lowercase()
    }

    /// Returns whether `ch` is an ASCII decimal digit.
    #[must_use]
    pub const fn is_number(ch: char) -> bool {
        ch.is_ascii_digit()
    }

    /// Returns whether `ch` is an ASCII hexadecimal digit.
    #[must_use]
    pub const fn is_hex_char(ch: char) -> bool {
        Self::is_number(ch) || matches!(ch, 'a'..='f' | 'A'..='F')
    }

    /// Returns whether `ch` is an ASCII letter or digit.
    #[must_use]
    pub const fn is_letter_or_number(ch: char) -> bool {
        Self::is_letter(ch) || Self::is_number(ch)
    }

    /// Converts one Unicode scalar to an owned string.
    #[must_use]
    pub fn to_string(ch: char) -> String {
        ch.to_string()
    }

    /// Returns whether `T` is Rust's character type.
    #[must_use]
    pub fn is_char_class<T: 'static>() -> bool {
        TypeId::of::<T>() == TypeId::of::<char>()
    }

    /// Returns whether a dynamically typed value is a Rust character.
    #[must_use]
    pub fn is_char(value: &dyn Any) -> bool {
        value.is::<char>()
    }

    /// Returns Hutool's extended blank-character classification.
    #[must_use]
    pub fn is_blank_char(ch: char) -> bool {
        Self::is_blank_code(ch as u32)
    }

    /// Integer-code-point overload of [`Self::is_blank_char`].
    #[must_use]
    pub fn is_blank_code(code: u32) -> bool {
        matches!(
            code,
            0x0000 | 0xfeff | 0x202a | 0x3164 | 0x2800 | 0x200c | 0x180e
        ) || char::from_u32(code).is_some_and(|ch| ch != '\u{0085}' && ch.is_whitespace())
    }

    /// Applies Hutool's broad UTF-16-era emoji heuristic to a Rust scalar.
    #[must_use]
    pub const fn is_emoji(ch: char) -> bool {
        let code = ch as u32;
        !matches!(
            code,
            0x0000 | 0x0009 | 0x000a | 0x000d | 0x0020..=0xd7ff | 0xe000..=0xfffd | 0x0010_0000..=0x0010_ffff
        )
    }

    /// Returns whether `ch` is a Unix or Windows path separator.
    #[must_use]
    pub const fn is_file_separator(ch: char) -> bool {
        matches!(ch, '/' | '\\')
    }

    /// Compares two characters, optionally using one-to-one lowercase mapping.
    #[must_use]
    pub fn equals(left: char, right: char, case_insensitive: bool) -> bool {
        if case_insensitive {
            lower_char(left) == lower_char(right)
        } else {
            left == right
        }
    }

    /// Returns the numeric constant used by Java `Character.getType`.
    #[must_use]
    pub fn get_type(code: u32) -> i32 {
        if (0xd800..=0xdfff).contains(&code) {
            return 19;
        }
        let Some(ch) = char::from_u32(code) else {
            return 0;
        };
        java_category(get_general_category(ch))
    }

    /// Returns the hexadecimal value of a Unicode code point, or `-1`.
    #[must_use]
    pub fn digit16(code: u32) -> i32 {
        let Some(ch) = char::from_u32(code) else {
            return -1;
        };
        if get_general_category(ch) == GeneralCategory::DecimalNumber {
            let mut start = code;
            while start > 0
                && char::from_u32(start - 1).is_some_and(|previous| {
                    get_general_category(previous) == GeneralCategory::DecimalNumber
                })
            {
                start -= 1;
            }
            return i32::try_from((code - start) % 10).unwrap_or(-1);
        }
        match ch {
            'A'..='F' => 10 + i32::from(ch as u8 - b'A'),
            'a'..='f' => 10 + i32::from(ch as u8 - b'a'),
            'Ａ'..='Ｆ' => 10 + (ch as i32 - 'Ａ' as i32),
            'ａ'..='ｆ' => 10 + (ch as i32 - 'ａ' as i32),
            _ => -1,
        }
    }

    /// Converts ASCII letters and digits 1 through 9 to enclosed characters.
    #[must_use]
    pub fn to_close_char(ch: char) -> char {
        let code = match ch {
            '1'..='9' => '①' as u32 + ch as u32 - '1' as u32,
            'A'..='Z' => 'Ⓐ' as u32 + ch as u32 - 'A' as u32,
            'a'..='z' => 'ⓐ' as u32 + ch as u32 - 'a' as u32,
            _ => return ch,
        };
        char::from_u32(code).unwrap_or(ch)
    }

    /// Converts a number in `1..=20` to its enclosed decimal form.
    #[allow(clippy::cast_sign_loss)]
    pub fn to_close_by_number(number: i32) -> Result<char, CharError> {
        if !(1..=20).contains(&number) {
            return Err(CharError::InvalidEnclosedNumber);
        }
        // The range guard proves this conversion cannot lose the sign.
        let number = number as u32;
        char::from_u32('①' as u32 + number - 1).ok_or(CharError::InvalidEnclosedNumber)
    }
}

use super::{java_category, java_category_abbreviation, lower_char};
