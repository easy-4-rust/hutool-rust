//! 对齐: `cn.hutool.core.date.LocalDateTimeUtil`

#![allow(dead_code)]

use chrono::{Datelike, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime, Timelike};

use crate::date::date_pattern::{self, NORM_DATE_PATTERN, NORM_TIME_PATTERN};
use crate::date::date_time::{DateTime, week_of_year_mon_min1};
use crate::date::date_unit::DateUnit;
use crate::{CoreError, Result};

/// 对齐 Java: `cn.hutool.core.date.LocalDateTimeUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct LocalDateTimeUtil;

impl LocalDateTimeUtil {
    /// 当前本地时间。
    pub fn now() -> NaiveDateTime {
        Local::now().naive_local()
    }

    /// 从 DateTime。
    pub fn of(dt: DateTime) -> NaiveDateTime {
        dt.naive_local()
    }

    /// 从 UTC 毫秒（不加本地偏移，纯 UTC naive）。
    pub fn of_utc(epoch_millis: i64) -> NaiveDateTime {
        chrono::DateTime::from_timestamp_millis(epoch_millis)
            .unwrap_or(chrono::DateTime::UNIX_EPOCH)
            .naive_utc()
    }

    /// 从 epoch 毫秒按本地 +08 墙钟。
    pub fn of_millis(epoch_millis: i64) -> NaiveDateTime {
        DateTime::of_millis(epoch_millis).naive_local()
    }

    /// 解析（自动 / 指定格式）。
    pub fn parse(date_str: &str) -> Result<NaiveDateTime> {
        let s = date_str.trim();
        if s.is_empty() {
            return Err(CoreError::InvalidArgument {
                name: "dateStr",
                reason: "blank",
            });
        }
        if let Ok(ndt) = NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.f") {
            return Ok(ndt);
        }
        if let Ok(ndt) = NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S") {
            return Ok(ndt);
        }
        if let Ok(ndt) = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S") {
            return Ok(ndt);
        }
        if let Ok(nd) = NaiveDate::parse_from_str(s, "%Y-%m-%d") {
            return Ok(nd.and_hms_opt(0, 0, 0).unwrap());
        }
        Err(CoreError::InvalidArgument {
            name: "dateStr",
            reason: "parse failed",
        })
    }

    /// 按格式解析。
    pub fn parse_format(date_str: &str, format: &str) -> Result<NaiveDateTime> {
        let s = date_str.trim();
        if s.chars().all(|c| c.is_ascii_digit()) {
            return parse_digits(s);
        }
        // Offset date-time: return local naive part
        if format.contains("XXX")
            || format.to_uppercase().contains("ISO_OFFSET")
            || s.contains('+')
            || s.ends_with('Z')
        {
            if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(s) {
                return Ok(dt.naive_local());
            }
            // try replacing
            if let Ok(dt) = chrono::DateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%z") {
                return Ok(dt.naive_local());
            }
        }
        if format == NORM_DATE_PATTERN || format == "yyyy-MM-dd" {
            let nd = NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(CoreError::from)?;
            return Ok(nd.and_hms_opt(0, 0, 0).unwrap());
        }
        if format == NORM_TIME_PATTERN || format == "HH:mm:ss" {
            let t = NaiveTime::parse_from_str(s, "%H:%M:%S").map_err(CoreError::from)?;
            // Hutool returns LocalDateTime with today's date; toString of toLocalTime is time only
            let today = Local::now().date_naive();
            return Ok(today.and_time(t));
        }
        let chrono_pat = date_pattern::to_chrono_format(format);
        if let Ok(ndt) = NaiveDateTime::parse_from_str(s, &chrono_pat) {
            return Ok(ndt);
        }
        // yyyyMMddHHmmssSSS
        if s.chars().all(|c| c.is_ascii_digit()) {
            return parse_digits(s);
        }
        // ISO_DATE_TIME
        Self::parse(s)
    }

    /// 解析日期字符串。
    ///
    /// # Errors
    ///
    /// 格式不合法时返回解析错误。
    pub fn parse_date(date_str: &str) -> Result<NaiveDate> {
        NaiveDate::parse_from_str(date_str.trim(), "%Y-%m-%d").map_err(CoreError::from)
    }

    /// 按格式格式化日期时间。
    pub fn format(dt: NaiveDateTime, pattern: &str) -> String {
        let chrono_pat = date_pattern::to_chrono_format(pattern);
        dt.format(&chrono_pat).to_string()
    }

    /// 按默认格式格式化日期时间。
    pub fn format_normal(dt: NaiveDateTime) -> String {
        dt.format("%Y-%m-%dT%H:%M:%S").to_string()
    }

    /// 按格式格式化日期。
    pub fn format_local_date(date: NaiveDate, pattern: &str) -> String {
        let chrono_pat = date_pattern::to_chrono_format(pattern);
        date.format(&chrono_pat).to_string()
    }

    /// 当天开始时刻。
    pub fn begin_of_day(dt: NaiveDateTime) -> NaiveDateTime {
        dt.date().and_hms_opt(0, 0, 0).unwrap()
    }
    /// 当天结束时刻。
    pub fn end_of_day(dt: NaiveDateTime) -> NaiveDateTime {
        dt.date().and_hms_nano_opt(23, 59, 59, 999_000_000).unwrap()
    }

    /// 按单位偏移。
    pub fn offset(dt: NaiveDateTime, unit: DateUnit, offset: i64) -> NaiveDateTime {
        match unit {
            DateUnit::Day => dt + Duration::days(offset),
            DateUnit::Hour => dt + Duration::hours(offset),
            DateUnit::Minute => dt + Duration::minutes(offset),
            DateUnit::Second => dt + Duration::seconds(offset),
            DateUnit::Ms => dt + Duration::milliseconds(offset),
            DateUnit::Week => dt + Duration::weeks(offset),
        }
    }

    /// 计算两时间在指定单位上的间隔。
    pub fn between(start: NaiveDateTime, end: NaiveDateTime, unit: DateUnit) -> i64 {
        let ms = (end - start).num_milliseconds();
        ms / unit.get_millis()
    }

    /// 两区间是否重叠。
    pub fn is_overlap(
        start1: NaiveDateTime,
        end1: NaiveDateTime,
        start2: NaiveDateTime,
        end2: NaiveDateTime,
    ) -> bool {
        start1 <= end2 && start2 <= end1
    }

    /// 一年中的周数。
    pub fn week_of_year(dt: NaiveDateTime) -> u32 {
        week_of_year_mon_min1(dt.date())
    }

    /// 对齐 Java: `LocalDateTimeUtil.toEpochMilli(TemporalAccessor)`
    pub fn to_epoch_milli(dt: NaiveDateTime) -> i64 {
        DateTime::of_naive(dt).get_time()
    }

    /// 对齐 Java: `LocalDateTimeUtil.isWeekend(LocalDateTime)`
    pub fn is_weekend(dt: NaiveDateTime) -> bool {
        matches!(dt.weekday(), chrono::Weekday::Sat | chrono::Weekday::Sun)
    }

    /// 对齐 Java: `LocalDateTimeUtil.isWeekend(LocalDate)`
    pub fn is_weekend_date(date: NaiveDate) -> bool {
        matches!(date.weekday(), chrono::Weekday::Sat | chrono::Weekday::Sun)
    }

    /// 对齐 Java: `LocalDateTimeUtil.isSameDay(LocalDateTime, LocalDateTime)`
    pub fn is_same_day(d1: NaiveDateTime, d2: NaiveDateTime) -> bool {
        d1.date() == d2.date()
    }

    /// 对齐 Java: `LocalDateTimeUtil.isSameDay(LocalDate, LocalDate)`
    pub fn is_same_day_date(d1: NaiveDate, d2: NaiveDate) -> bool {
        d1 == d2
    }

    /// 对齐 Java: `LocalDateTimeUtil.isIn(date, begin, end)`（闭区间）
    pub fn is_in(date: NaiveDateTime, begin: NaiveDateTime, end: NaiveDateTime) -> bool {
        date >= begin && date <= end
    }

    /// 对齐 Java: `LocalDateTimeUtil.of(Date)` / DateTime
    pub fn of_date(dt: DateTime) -> NaiveDateTime {
        Self::of(dt)
    }

    /// 对齐 Java: `LocalDateTimeUtil.ofDate(TemporalAccessor)` → LocalDate
    pub fn of_local_date(dt: NaiveDateTime) -> NaiveDate {
        dt.date()
    }

    /// 对齐 Java: `LocalDateTimeUtil.dayOfWeek(LocalDate)`
    pub fn day_of_week(date: NaiveDate) -> chrono::Weekday {
        date.weekday()
    }

    /// 对齐 Java: `LocalDateTimeUtil.betweenPeriod`（日期间隔天数）
    pub fn between_period_days(start: NaiveDate, end: NaiveDate) -> i64 {
        (end - start).num_days()
    }

    /// 兼容 sentinel。
    pub fn sentinel() -> Result<()> {
        Ok(())
    }
}

fn parse_digits(s: &str) -> Result<NaiveDateTime> {
    match s.len() {
        14 => {
            let y: i32 = s[0..4].parse().unwrap();
            let m: u32 = s[4..6].parse().unwrap();
            let d: u32 = s[6..8].parse().unwrap();
            let h: u32 = s[8..10].parse().unwrap();
            let mi: u32 = s[10..12].parse().unwrap();
            let sec: u32 = s[12..14].parse().unwrap();
            NaiveDate::from_ymd_opt(y, m, d)
                .and_then(|d| d.and_hms_opt(h, mi, sec))
                .ok_or(CoreError::DateOverflow)
        }
        17 => {
            let base = parse_digits(&s[..14])?;
            let mut frac = s[14..].to_string();
            while frac.len() < 3 {
                frac.push('0');
            }
            let ms: u32 = frac[..3].parse().unwrap_or(0);
            Ok(base.with_nanosecond(ms * 1_000_000).unwrap())
        }
        16 => {
            // SS = 2 digit fractional -> *10 ms
            let base = parse_digits(&s[..14])?;
            let frac: u32 = s[14..].parse().unwrap_or(0);
            Ok(base.with_nanosecond(frac * 10 * 1_000_000).unwrap())
        }
        _ => Err(CoreError::InvalidArgument {
            name: "dateStr",
            reason: "digit length",
        }),
    }
}
