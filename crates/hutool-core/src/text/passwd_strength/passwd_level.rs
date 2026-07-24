//! 对齐: `cn.hutool.core.text.PasswdStrength`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/PasswdStrength.java
//!
//! 密码强度检测。

use crate::Result;

use super::passwd_strength::PasswdStrength;

/// 对齐 Java: `PasswdStrength#PASSWD_LEVEL` 枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PasswdLevel {
    Easy,
    Medium,
    Strong,
    VeryStrong,
    ExtremelyStrong,
}

use super::{DICTIONARY, SIZE_TABLE, check_character_type, count_letter, is_char_equals, is_numeric, size_of_int};
