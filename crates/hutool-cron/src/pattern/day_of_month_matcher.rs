#![allow(clippy::missing_panics_doc)]
//! 对齐: `cn.hutool.cron.pattern.matcher.DayOfMonthMatcher`
//! 来源: hutool-cron/src/main/java/cn/hutool/cron/pattern/matcher/DayOfMonthMatcher.java
//! 中文说明: 支持 Hutool `L` 哨兵值（32 表示月末）的日期匹配器。

use std::{fmt, str::FromStr};

use chrono::{DateTime, Datelike, Duration as ChronoDuration, TimeZone, Timelike, Utc};
use cron::Schedule;

use crate::CronError;

use super::bool_array_matcher::BoolArrayMatcher;
use super::part::Part;
use super::part_matcher::PartMatcher;

/// 对齐: `cn.hutool.cron.pattern.matcher.DayOfMonthMatcher`
/// 中文说明: 支持 Hutool `L` 哨兵值（32 表示月末）的日期匹配器。
///
/// Day-of-month matcher supporting Hutool's `L` sentinel (`32`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DayOfMonthMatcher {
    values: BoolArrayMatcher,
    last: bool,
}

impl DayOfMonthMatcher {
    /// 中文说明: 创建日期匹配器，值 `32` 表示月末。
    /// 对齐 Java 方法: `new`
    pub fn new(values: impl IntoIterator<Item = i32>) -> Result<Self, CronError> {
        Self::from_values(values.into_iter().collect())
    }

    pub(crate) fn from_values(values: Vec<i32>) -> Result<Self, CronError> {
        let mut last = false;
        let mut concrete = Vec::new();
        for value in values {
            if value == 32 {
                last = true;
            } else {
                concrete.push(Part::DayOfMonth.check_value(value)?);
            }
        }
        if concrete.is_empty() {
            concrete.push(32);
        }
        Ok(Self {
            values: BoolArrayMatcher::new(concrete)
                .expect("a day matcher always contains a concrete value or the last-day sentinel"),
            last,
        })
    }

    /// 中文说明: 返回是否启用了月末哨兵。
    /// 对齐 Java 方法: `isLast`
    #[must_use]
    pub const fn is_last(&self) -> bool {
        self.last
    }

    /// 中文说明: 返回指定月份的天数。
    /// 对齐 Java 方法: `getLastDay`
    #[must_use]
    pub const fn last_day(month: u32, leap_year: bool) -> u32 {
        match month {
            2 if leap_year => 29,
            2 => 28,
            4 | 6 | 9 | 11 => 30,
            _ => 31,
        }
    }

    /// 中文说明: 根据月份和闰年上下文匹配日期。
    /// 对齐 Java 方法: `match`
    #[must_use]
    pub fn matches_day(&self, day: u32, month: u32, leap_year: bool) -> bool {
        self.values.matches(i32::try_from(day).unwrap_or(i32::MAX))
            || (self.last && day == Self::last_day(month, leap_year))
    }

    /// 中文说明: 返回月份内的下一个匹配日期，或最小匹配值。
    /// 对齐 Java 方法: `nextDay`
    #[must_use]
    pub fn next_day(&self, day: u32, month: u32, leap_year: bool) -> u32 {
        let last_day = Self::last_day(month, leap_year);
        (day..=last_day)
            .find(|candidate| self.matches_day(*candidate, month, leap_year))
            .or_else(|| {
                (1..=last_day).find(|candidate| self.matches_day(*candidate, month, leap_year))
            })
            .unwrap_or(day)
    }

    /// 中文说明: 返回月份内的最小具体匹配值。
    /// 对齐 Java 方法: `getMinValue`
    #[must_use]
    pub fn min_value(&self, month: u32, leap_year: bool) -> u32 {
        self.next_day(1, month, leap_year)
    }

    /// 中文说明: 返回月份内的最大具体匹配值。
    /// 对齐 Java 方法: `getMaxValue`
    #[must_use]
    pub fn max_value(&self, month: u32, leap_year: bool) -> u32 {
        let last_day = Self::last_day(month, leap_year);
        (1..=last_day)
            .rev()
            .find(|candidate| self.matches_day(*candidate, month, leap_year))
            .unwrap_or(last_day)
    }
}

impl PartMatcher for DayOfMonthMatcher {
    fn matches(&self, value: i32) -> bool {
        self.values.matches(value) || (self.last && value == 32)
    }

    fn next_after(&self, value: i32) -> i32 {
        self.values.next_after(value)
    }
}

use super::split_numeric_range;
use super::{
    apply_negative, checked_schedule_value, convert_hutool_dow_field, convert_hutool_dow_token,
    end_of_year, expand_field, expand_range, field_needs_expand,
};
use super::{
    fields, hutool_dow_to_quartz, is_last_day_of_month, next_after_filtered, normalize_expanded,
    pad_fields, parse_alias, schedule_max,
};
