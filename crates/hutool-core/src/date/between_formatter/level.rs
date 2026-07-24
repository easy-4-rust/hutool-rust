//! 对齐: `cn.hutool.core.date.BetweenFormatter`

#![allow(dead_code)]

use crate::date::date_unit::DateUnit;

use super::between_formatter::BetweenFormatter;

/// 对齐 Java: `BetweenFormatter.Level`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Level {
    Day = 0,
    Hour = 1,
    Minute = 2,
    Second = 3,
    Millisecond = 4,
}

impl Level {
    /// 中文单位名。
    pub fn get_name(self) -> &'static str {
        match self {
            Self::Day => "天",
            Self::Hour => "小时",
            Self::Minute => "分",
            Self::Second => "秒",
            Self::Millisecond => "毫秒",
        }
    }
}
