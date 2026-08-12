//! 对齐: `cn.hutool.core.lang.ansi.AnsiColor`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/ansi/AnsiColor.java
//!
//! 状态: 对齐桩,等待完整实现。

#![allow(dead_code, unused_variables, clippy::new_without_default)]

/// 对齐 Java enum: `cn.hutool.core.lang.ansi.AnsiColor`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)] // 对齐 Java 枚举常量名（BRIGHT_BLACK 等）
pub enum AnsiColor {
    /// 对齐 Java 枚举常量: `DEFAULT`
    DEFAULT,
    /// 对齐 Java 枚举常量: `BLACK`
    BLACK,
    /// 对齐 Java 枚举常量: `RED`
    RED,
    /// 对齐 Java 枚举常量: `GREEN`
    GREEN,
    /// 对齐 Java 枚举常量: `YELLOW`
    YELLOW,
    /// 对齐 Java 枚举常量: `BLUE`
    BLUE,
    /// 对齐 Java 枚举常量: `MAGENTA`
    MAGENTA,
    /// 对齐 Java 枚举常量: `CYAN`
    CYAN,
    /// 对齐 Java 枚举常量: `WHITE`
    WHITE,
    /// 对齐 Java 枚举常量: `BRIGHT_BLACK`
    BRIGHT_BLACK,
    /// 对齐 Java 枚举常量: `BRIGHT_RED`
    BRIGHT_RED,
    /// 对齐 Java 枚举常量: `BRIGHT_GREEN`
    BRIGHT_GREEN,
    /// 对齐 Java 枚举常量: `BRIGHT_YELLOW`
    BRIGHT_YELLOW,
    /// 对齐 Java 枚举常量: `BRIGHT_BLUE`
    BRIGHT_BLUE,
    /// 对齐 Java 枚举常量: `BRIGHT_MAGENTA`
    BRIGHT_MAGENTA,
    /// 对齐 Java 枚举常量: `BRIGHT_CYAN`
    BRIGHT_CYAN,
    /// 对齐 Java 枚举常量: `BRIGHT_WHITE`
    BRIGHT_WHITE,
}

impl AnsiColor {
    /// ANSI 前景色代码
    pub fn code(self) -> u8 {
        match self {
            Self::DEFAULT => 39,
            Self::BLACK => 30,
            Self::RED => 31,
            Self::GREEN => 32,
            Self::YELLOW => 33,
            Self::BLUE => 34,
            Self::MAGENTA => 35,
            Self::CYAN => 36,
            Self::WHITE => 37,
            Self::BRIGHT_BLACK => 90,
            Self::BRIGHT_RED => 91,
            Self::BRIGHT_GREEN => 92,
            Self::BRIGHT_YELLOW => 93,
            Self::BRIGHT_BLUE => 94,
            Self::BRIGHT_MAGENTA => 95,
            Self::BRIGHT_CYAN => 96,
            Self::BRIGHT_WHITE => 97,
        }
    }
}
