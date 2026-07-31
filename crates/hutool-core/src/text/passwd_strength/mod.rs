//! 对齐: `cn.hutool.core.text.PasswdStrength`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/PasswdStrength.java
//!
//! 密码强度检测。

mod passwd_level;
mod char_type;
mod passwd_strength;

pub use passwd_level::PasswdLevel;
pub use char_type::CharType;
pub use passwd_strength::PasswdStrength;

const DICTIONARY: &[&str] = &[
    "password", "abc123", "iloveyou", "adobe123", "123123", "sunshine", "1314520", "a1b2c3",
    "123qwe", "aaa111", "qweasd", "admin", "passwd",
];

const SIZE_TABLE: &[i32] = &[
    9, 99, 999, 9999, 99999, 999999, 9999999, 99999999, 999999999, i32::MAX,
];

fn check_character_type(c: char) -> CharType {
    match c as u32 {
        48..=57 => CharType::Num,
        65..=90 => CharType::CapitalLetter,
        97..=122 => CharType::SmallLetter,
        _ => CharType::OtherChar,
    }
}

fn count_letter(passwd: &str, ty: CharType) -> i32 {
    passwd
        .chars()
        .filter(|c| check_character_type(*c) == ty)
        .count() as i32
}

fn is_numeric(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_digit())
}

fn is_char_equals(s: &str) -> bool {
    let mut chars = s.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    chars.all(|c| c == first)
}

fn size_of_int(x: i32) -> i32 {
    for (i, &bound) in SIZE_TABLE.iter().enumerate() {
        if x <= bound {
            return (i + 1) as i32;
        }
    }
    10
}
