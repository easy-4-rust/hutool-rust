//! 对齐: `cn.hutool.core.date.DateTime`
//!
//! # Timezone note
//! 默认按 UTC+08:00（Asia/Shanghai 无夏令时偏移）解释/格式化墙钟时间，
//! 与 Hutool `DateUtilTest` 的 `TZ=Asia/Shanghai` 约定一致。

#![allow(dead_code)]

use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, Timelike, Weekday};

use crate::date::date_field::DateField;
use crate::date::date_pattern::NORM_DATETIME_PATTERN;
use crate::date::date_unit::DateUnit;
use crate::date::month::Month;
use crate::date::quarter::Quarter;
use crate::date::week::Week;
use crate::{CoreError, Result};

use super::date_time::DateTime;

/// 兼容旧占位名。
pub type HutoolDateTime = DateTime;

impl Default for DateTime {
    fn default() -> Self {
        Self::now()
    }
}

impl PartialEq for DateTime {
    fn eq(&self, other: &Self) -> bool {
        self.millis == other.millis
    }
}

impl Eq for DateTime {}

impl PartialOrd for DateTime {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DateTime {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.millis.cmp(&other.millis)
    }
}

impl DateTime {
    /// 当前时间。
    pub fn now() -> Self {
        Self::of_millis(chrono::Utc::now().timestamp_millis())
    }

    /// 从 epoch 毫秒构造。
    pub fn of_millis(millis: i64) -> Self {
        Self {
            millis,
            first_day_of_week: Week::Monday,
            mutable: true,
        }
    }

    /// 从本地墙钟 NaiveDateTime（按 +08:00）构造。
    pub fn of_naive(naive: NaiveDateTime) -> Self {
        let millis = naive
            .and_local_timezone(parity_zone())
            .single()
            .map(|d| d.timestamp_millis())
            .unwrap_or_else(|| {
                let utc = naive - Duration::hours(8);
                utc.and_utc().timestamp_millis()
            });
        Self::of_millis(millis)
    }

    /// 从年月日时分秒（墙钟 +08）构造。
    pub fn of_ymd_hms(y: i32, month: u32, day: u32, h: u32, mi: u32, s: u32) -> Result<Self> {
        let naive = NaiveDate::from_ymd_opt(y, month, day)
            .and_then(|d| d.and_hms_opt(h, mi, s))
            .ok_or(CoreError::DateOverflow)?;
        Ok(Self::of_naive(naive))
    }

    /// epoch 毫秒。
    pub fn get_time(self) -> i64 {
        self.millis
    }

    /// 设置一周起始。
    pub fn set_first_day_of_week(&mut self, week: Week) {
        self.first_day_of_week = week;
    }

    /// 获取一周起始。
    pub fn first_day_of_week(self) -> Week {
        self.first_day_of_week
    }

    /// 设置可变。
    pub fn set_mutable(&mut self, mutable: bool) {
        self.mutable = mutable;
    }

    /// 是否可变。
    pub fn is_mutable(self) -> bool {
        self.mutable
    }

    /// 墙钟 NaiveDateTime（+08:00）。
    pub fn naive_local(self) -> NaiveDateTime {
        chrono::DateTime::from_timestamp_millis(self.millis)
            .unwrap_or_else(|| chrono::DateTime::UNIX_EPOCH)
            .with_timezone(&parity_zone())
            .naive_local()
    }

    /// 年。
    pub fn year(self) -> i32 {
        self.naive_local().year()
    }

    /// 月（0-11，对齐 Hutool/Calendar）。
    pub fn month(self) -> i32 {
        self.naive_local().month0() as i32
    }

    /// 月枚举。
    pub fn month_enum(self) -> Month {
        Month::of_value(self.month()).unwrap_or(Month::January)
    }

    /// 日（1-31）。
    pub fn day_of_month(self) -> i32 {
        self.naive_local().day() as i32
    }

    /// 一年中的第几天。
    pub fn day_of_year(self) -> i32 {
        self.naive_local().ordinal() as i32
    }

    /// 季度 1-4。
    pub fn quarter(self) -> i32 {
        ((self.naive_local().month() - 1) / 3 + 1) as i32
    }

    /// 季度枚举。
    pub fn quarter_enum(self) -> Quarter {
        Quarter::of(self.quarter()).unwrap()
    }

    /// 时（24h）。
    pub fn hour(self, is_24: bool) -> i32 {
        let h = self.naive_local().hour() as i32;
        if is_24 { h } else { h % 12 }
    }

    /// 分。
    pub fn minute(self) -> i32 {
        self.naive_local().minute() as i32
    }

    /// 秒。
    pub fn second(self) -> i32 {
        self.naive_local().second() as i32
    }

    /// 毫秒。
    pub fn millisecond(self) -> i32 {
        (self.naive_local().nanosecond() / 1_000_000) as i32
    }

    /// 周几（Calendar：1=周日）。
    pub fn day_of_week(self) -> i32 {
        Week::of_weekday(self.naive_local().weekday()).get_value()
    }

    /// 周几枚举。
    pub fn day_of_week_enum(self) -> Week {
        Week::of_weekday(self.naive_local().weekday())
    }

    /// 是否周末。
    pub fn is_weekend(self) -> bool {
        matches!(self.naive_local().weekday(), Weekday::Sat | Weekday::Sun)
    }

    /// 一年中第几周（Hutool 默认：周一为一周起点、首周最少 1 天）。
    pub fn week_of_year(self) -> u32 {
        week_of_year_mon_min1(self.naive_local().date())
    }

    /// 对齐 Java: `DateTime.weekOfMonth()`（当月第几周，周一为一周起点）
    pub fn week_of_month(self) -> u32 {
        let d = self.naive_local().date();
        let first = NaiveDate::from_ymd_opt(d.year(), d.month(), 1).unwrap();
        let first_from_mon = first.weekday().num_days_from_monday();
        ((d.day() + first_from_mon - 1) / 7) + 1
    }

    /// 对齐 Java: `DateTime.isAM()`
    pub fn is_am(self) -> bool {
        self.hour(true) < 12
    }

    /// 对齐 Java: `DateTime.isPM()`
    pub fn is_pm(self) -> bool {
        !self.is_am()
    }

    /// 对齐 Java: `DateTime.isLeapYear()`
    pub fn is_leap_year(self) -> bool {
        NaiveDate::from_ymd_opt(self.year(), 2, 29).is_some()
    }

    /// 对齐 Java: `DateTime.isAfter(Date)`
    pub fn is_after(self, other: Self) -> bool {
        self.get_time() > other.get_time()
    }

    /// 对齐 Java: `DateTime.isBefore(Date)`
    pub fn is_before(self, other: Self) -> bool {
        self.get_time() < other.get_time()
    }

    /// 对齐 Java: `DateTime.isAfterOrEquals(Date)`
    pub fn is_after_or_equals(self, other: Self) -> bool {
        self.get_time() >= other.get_time()
    }

    /// 对齐 Java: `DateTime.isBeforeOrEquals(Date)`
    pub fn is_before_or_equals(self, other: Self) -> bool {
        self.get_time() <= other.get_time()
    }

    /// 对齐 Java: `DateTime.isIn(Date, Date)`
    pub fn is_in(self, begin: Self, end: Self) -> bool {
        self.get_time() >= begin.get_time() && self.get_time() <= end.get_time()
    }

    /// 对齐 Java: `DateTime.isLastDayOfMonth()`
    pub fn is_last_day_of_month(self) -> bool {
        let n = self.naive_local();
        let tomorrow = n.date() + Duration::days(1);
        tomorrow.month() != n.month()
    }

    /// 对齐 Java: `DateTime.monthStartFromOne()` / `monthBaseOne`
    pub fn month_base_one(self) -> i32 {
        self.naive_local().month() as i32
    }

    /// 对齐 Java: `DateTime.offsetNew(DateField, int)`（不可变偏移）
    pub fn offset_new(self, field: DateField, offset: i64) -> Self {
        self.offset(field, offset)
    }

    /// 对齐 Java: `DateTime.between(Date, DateUnit)`
    pub fn between(self, other: Self, unit: DateUnit) -> i64 {
        between_unit(self, other, unit).abs()
    }

    /// 格式化为默认 `yyyy-MM-dd HH:mm:ss`。
    pub fn to_string_normal(self) -> String {
        self.format(NORM_DATETIME_PATTERN)
    }

    /// 按 Hutool pattern 格式化。
    pub fn format(self, pattern: &str) -> String {
        format_with_pattern(self, pattern)
    }

    /// 偏移字段。
    pub fn offset(self, field: DateField, offset: i64) -> Self {
        let naive = self.naive_local();
        let new_naive = match field {
            DateField::Year => {
                let y = naive.year() as i64 + offset;
                naive.with_year(y as i32).unwrap_or(naive)
            }
            DateField::Month => {
                let total = naive.year() as i64 * 12 + naive.month0() as i64 + offset;
                let y = (total.div_euclid(12)) as i32;
                let m = (total.rem_euclid(12) + 1) as u32;
                let day = naive.day().min(days_in_month(y, m));
                NaiveDate::from_ymd_opt(y, m, day)
                    .and_then(|d| {
                        d.and_hms_nano_opt(
                            naive.hour(),
                            naive.minute(),
                            naive.second(),
                            naive.nanosecond(),
                        )
                    })
                    .unwrap_or(naive)
            }
            DateField::WeekOfYear | DateField::WeekOfMonth => naive + Duration::weeks(offset),
            DateField::DayOfMonth | DateField::DayOfYear | DateField::DayOfWeek => {
                naive + Duration::days(offset)
            }
            DateField::Hour | DateField::HourOfDay => naive + Duration::hours(offset),
            DateField::Minute => naive + Duration::minutes(offset),
            DateField::Second => naive + Duration::seconds(offset),
            DateField::Millisecond => naive + Duration::milliseconds(offset),
            DateField::AmPm => naive + Duration::hours(offset * 12),
            DateField::Era | DateField::DayOfWeekInMonth => naive,
        };
        let mut dt = Self::of_naive(new_naive);
        dt.first_day_of_week = self.first_day_of_week;
        dt.mutable = self.mutable;
        dt
    }

    /// 兼容 sentinel。
    pub fn sentinel() -> Result<()> {
        Ok(())
    }
}

impl std::fmt::Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string_normal())
    }
}

use super::{between_unit, days_in_month, format_with_pattern, parity_zone, week_of_year_mon_min1};
