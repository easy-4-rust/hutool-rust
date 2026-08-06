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
            // 对齐 Java Part.DAY_OF_WEEK(SUNDAY.ordinal(), SATURDAY.ordinal()) = 0..6；
            // 表达式中的 7 由 check_value 拒绝，"0 和 7 都表示周日" 的 7 仅存于
            // PatternMatcher::matches_week 的防御性查询分支。
            Self::DayOfWeek => 6,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_calendar_fields_and_ranges() {
        assert_eq!(Part::Second.calendar_field(), 0);
        assert_eq!(Part::Year.calendar_field(), 6);
        assert_eq!(Part::Second.min(), 0);
        assert_eq!(Part::Second.max(), 59);
        assert_eq!(Part::DayOfMonth.min(), 1);
        // 对齐 Java：DAY_OF_MONTH(1, 32)，32 为 L（月末）哨兵
        assert_eq!(Part::DayOfMonth.max(), 32);
        assert_eq!(Part::Year.min(), 1970);
        assert_eq!(Part::Year.max(), 2099);
    }

    #[test]
    fn part_check_value_bounds() {
        assert_eq!(Part::Hour.check_value(23).unwrap(), 23);
        assert!(Part::Hour.check_value(24).is_err());
        assert!(Part::Hour.check_value(-1).is_err());
        // DayOfMonth 允许 32（L 哨兵）
        assert_eq!(Part::DayOfMonth.check_value(32).unwrap(), 32);
        // 对齐 Java：DayOfWeek 范围为 0..=6，7 被拒绝（"7" 仅在 matchWeek 防御分支出现）
        assert!(Part::DayOfWeek.check_value(7).is_err());
        assert!(Part::DayOfWeek.check_value(8).is_err());
    }

    #[test]
    fn part_of_maps_index() {
        assert_eq!(Part::of(0).unwrap(), Part::Second);
        assert_eq!(Part::of(6).unwrap(), Part::Year);
        assert!(Part::of(7).is_err());
    }
}
