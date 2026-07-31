#![allow(clippy::missing_panics_doc)]
//! 对齐: `cn.hutool.cron.pattern.Part`
//! 来源: hutool-cron/src/main/java/cn/hutool/cron/pattern/Part.java
//! 中文说明: Cron 表达式字段枚举，定义秒、分、时、日、月、周、年各字段的取值范围。

use crate::CronError;

/// 对齐: `cn.hutool.cron.pattern.Part`
/// 中文说明: Hutool cron 表达式中的字段类型。
///
/// A field in a Hutool cron expression.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Part {
    /// Seconds, `0..=59`.
    Second,
    /// Minutes, `0..=59`.
    Minute,
    /// Hours, `0..=23`.
    Hour,
    /// Day of month, `1..=31`, with Hutool sentinel `32` for `L` (last day).
    DayOfMonth,
    /// Month, `1..=12`.
    Month,
    /// Day of week, `0..=7`.
    DayOfWeek,
    /// Year, `1970..=2099`.
    Year,
}

impl Part {
    /// 中文说明: 返回 Hutool/Calendar 字段索引。
    /// 对齐 Java 方法: `getCalendarField`
    #[must_use]
    pub const fn calendar_field(self) -> usize {
        match self {
            Self::Second => 0,
            Self::Minute => 1,
            Self::Hour => 2,
            Self::DayOfMonth => 3,
            Self::Month => 4,
            Self::DayOfWeek => 5,
            Self::Year => 6,
        }
    }

    /// 中文说明: 返回字段的最小值（含）。
    /// 对齐 Java 方法: `getMin`
    #[must_use]
    pub const fn min(self) -> i32 {
        match self {
            Self::Second | Self::Minute | Self::Hour | Self::DayOfWeek => 0,
            Self::DayOfMonth | Self::Month => 1,
            Self::Year => 1970,
        }
    }

    /// 中文说明: 返回字段的最大值（含）。
    /// 对齐 Java 方法: `getMax`
    #[must_use]
    pub const fn max(self) -> i32 {
        match self {
            Self::Second | Self::Minute => 59,
            Self::Hour => 23,
            Self::DayOfMonth => 32, // Hutool: 32 == last day ("L")
            Self::Month => 12,
            Self::DayOfWeek => 7,
            Self::Year => 2099,
        }
    }

    /// 中文说明: 验证并返回字段值。
    /// 对齐 Java 方法: `checkValue`
    pub fn check_value(self, value: i32) -> Result<i32, CronError> {
        if (self.min()..=self.max()).contains(&value) {
            Ok(value)
        } else {
            Err(CronError::InvalidPartValue { part: self, value })
        }
    }

    /// 中文说明: 按零基索引解析字段。
    /// 对齐 Java 方法: `of`
    pub fn of(index: usize) -> Result<Self, CronError> {
        [
            Self::Second,
            Self::Minute,
            Self::Hour,
            Self::DayOfMonth,
            Self::Month,
            Self::DayOfWeek,
            Self::Year,
        ]
        .get(index)
        .copied()
        .ok_or(CronError::InvalidPartIndex(index))
    }
}
