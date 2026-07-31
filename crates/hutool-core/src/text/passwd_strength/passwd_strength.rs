//! 对齐: `cn.hutool.core.text.PasswdStrength`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/PasswdStrength.java
//!
//! 密码强度检测。

use crate::Result;

use super::char_type::CharType;
use super::passwd_level::PasswdLevel;

/// 对齐 Java: `PasswdStrength#`
#[derive(Debug, Clone, Copy, Default)]
pub struct PasswdStrength;

impl PasswdStrength {
    /// 对齐 Java: `PasswdStrength::check#int (String passwd)`
    pub fn check(passwd: &str) -> Result<i32> {
        let len = passwd.chars().count() as i32;
        let mut level = 0i32;

        if count_letter(passwd, CharType::Num) > 0 {
            level += 1;
        }
        if count_letter(passwd, CharType::SmallLetter) > 0 {
            level += 1;
        }
        if len > 4 && count_letter(passwd, CharType::CapitalLetter) > 0 {
            level += 1;
        }
        if len > 6 && count_letter(passwd, CharType::OtherChar) > 0 {
            level += 1;
        }

        if (len > 4
            && count_letter(passwd, CharType::Num) > 0
            && count_letter(passwd, CharType::SmallLetter) > 0)
            || (count_letter(passwd, CharType::Num) > 0
                && count_letter(passwd, CharType::CapitalLetter) > 0)
            || (count_letter(passwd, CharType::Num) > 0
                && count_letter(passwd, CharType::OtherChar) > 0)
            || (count_letter(passwd, CharType::SmallLetter) > 0
                && count_letter(passwd, CharType::CapitalLetter) > 0)
            || (count_letter(passwd, CharType::SmallLetter) > 0
                && count_letter(passwd, CharType::OtherChar) > 0)
            || (count_letter(passwd, CharType::CapitalLetter) > 0
                && count_letter(passwd, CharType::OtherChar) > 0)
        {
            level += 1;
        }

        if (len > 6
            && count_letter(passwd, CharType::Num) > 0
            && count_letter(passwd, CharType::SmallLetter) > 0
            && count_letter(passwd, CharType::CapitalLetter) > 0)
            || (count_letter(passwd, CharType::Num) > 0
                && count_letter(passwd, CharType::SmallLetter) > 0
                && count_letter(passwd, CharType::OtherChar) > 0)
            || (count_letter(passwd, CharType::Num) > 0
                && count_letter(passwd, CharType::CapitalLetter) > 0
                && count_letter(passwd, CharType::OtherChar) > 0)
            || (count_letter(passwd, CharType::SmallLetter) > 0
                && count_letter(passwd, CharType::CapitalLetter) > 0
                && count_letter(passwd, CharType::OtherChar) > 0)
        {
            level += 1;
        }

        if len > 8
            && count_letter(passwd, CharType::Num) > 0
            && count_letter(passwd, CharType::SmallLetter) > 0
            && count_letter(passwd, CharType::CapitalLetter) > 0
            && count_letter(passwd, CharType::OtherChar) > 0
        {
            level += 1;
        }

        if (len > 6
            && count_letter(passwd, CharType::Num) >= 3
            && count_letter(passwd, CharType::SmallLetter) >= 3)
            || (count_letter(passwd, CharType::Num) >= 3
                && count_letter(passwd, CharType::CapitalLetter) >= 3)
            || (count_letter(passwd, CharType::Num) >= 3
                && count_letter(passwd, CharType::OtherChar) >= 2)
            || (count_letter(passwd, CharType::SmallLetter) >= 3
                && count_letter(passwd, CharType::CapitalLetter) >= 3)
            || (count_letter(passwd, CharType::SmallLetter) >= 3
                && count_letter(passwd, CharType::OtherChar) >= 2)
            || (count_letter(passwd, CharType::CapitalLetter) >= 3
                && count_letter(passwd, CharType::OtherChar) >= 2)
        {
            level += 1;
        }

        if (len > 8
            && count_letter(passwd, CharType::Num) >= 2
            && count_letter(passwd, CharType::SmallLetter) >= 2
            && count_letter(passwd, CharType::CapitalLetter) >= 2)
            || (count_letter(passwd, CharType::Num) >= 2
                && count_letter(passwd, CharType::SmallLetter) >= 2
                && count_letter(passwd, CharType::OtherChar) >= 2)
            || (count_letter(passwd, CharType::Num) >= 2
                && count_letter(passwd, CharType::CapitalLetter) >= 2
                && count_letter(passwd, CharType::OtherChar) >= 2)
            || (count_letter(passwd, CharType::SmallLetter) >= 2
                && count_letter(passwd, CharType::CapitalLetter) >= 2
                && count_letter(passwd, CharType::OtherChar) >= 2)
        {
            level += 1;
        }

        if len > 10
            && count_letter(passwd, CharType::Num) >= 2
            && count_letter(passwd, CharType::SmallLetter) >= 2
            && count_letter(passwd, CharType::CapitalLetter) >= 2
            && count_letter(passwd, CharType::OtherChar) >= 2
        {
            level += 1;
        }

        if count_letter(passwd, CharType::OtherChar) >= 3 {
            level += 1;
        }
        if count_letter(passwd, CharType::OtherChar) >= 6 {
            level += 1;
        }

        if len > 12 {
            level += 1;
            if len >= 16 {
                level += 1;
            }
        }

        if "abcdefghijklmnopqrstuvwxyz".contains(passwd)
            || "ABCDEFGHIJKLMNOPQRSTUVWXYZ".contains(passwd)
        {
            level -= 1;
        }
        if "qwertyuiop".contains(passwd)
            || "asdfghjkl".contains(passwd)
            || "zxcvbnm".contains(passwd)
        {
            level -= 1;
        }
        if is_numeric(passwd)
            && ("01234567890".contains(passwd) || "09876543210".contains(passwd))
        {
            level -= 1;
        }

        if count_letter(passwd, CharType::Num) == len
            || count_letter(passwd, CharType::SmallLetter) == len
            || count_letter(passwd, CharType::CapitalLetter) == len
        {
            level -= 1;
        }

        if len % 2 == 0 {
            let mid = (len / 2) as usize;
            let chars: Vec<char> = passwd.chars().collect();
            let part1: String = chars[..mid].iter().collect();
            let part2: String = chars[mid..].iter().collect();
            if part1 == part2 {
                level -= 1;
            }
            if is_char_equals(&part1) && is_char_equals(&part2) {
                level -= 1;
            }
        }
        if len % 3 == 0 {
            let third = (len / 3) as usize;
            let chars: Vec<char> = passwd.chars().collect();
            let part1: String = chars[..third].iter().collect();
            let part2: String = chars[third..third * 2].iter().collect();
            let part3: String = chars[third * 2..].iter().collect();
            if part1 == part2 && part2 == part3 {
                level -= 1;
            }
        }

        if is_numeric(passwd) && (6..=8).contains(&len) {
            let year = if len == 8 || len == 6 {
                passwd[..(len as usize - 4)].parse::<i32>().unwrap_or(0)
            } else {
                0
            };
            let size = size_of_int(year) as usize;
            let month: i32 = passwd[size..size + 2].parse().unwrap_or(0);
            let day: i32 = passwd[size + 2..len as usize].parse().unwrap_or(0);
            if (1950..2050).contains(&year) && (1..=12).contains(&month) && (1..=31).contains(&day)
            {
                level -= 1;
            }
        }

        for s in DICTIONARY {
            if passwd == *s || s.contains(passwd) {
                level -= 1;
                break;
            }
        }

        if len <= 6 {
            level -= 1;
            if len <= 4 {
                level -= 1;
                if len <= 3 {
                    level = 0;
                }
            }
        }

        if is_char_equals(passwd) {
            level = 0;
        }
        if level < 0 {
            level = 0;
        }
        Ok(level)
    }

    /// 对齐 Java: `PasswdStrength::getLevel#PASSWD_LEVEL (String passwd)`
    pub fn get_level(passwd: &str) -> Result<PasswdLevel> {
        Ok(match Self::check(passwd)? {
            0 | 1 | 2 | 3 => PasswdLevel::Easy,
            4 | 5 | 6 => PasswdLevel::Medium,
            7 | 8 | 9 => PasswdLevel::Strong,
            10 | 11 | 12 => PasswdLevel::VeryStrong,
            _ => PasswdLevel::ExtremelyStrong,
        })
    }
}

use super::{DICTIONARY, count_letter, is_char_equals, is_numeric, size_of_int};
