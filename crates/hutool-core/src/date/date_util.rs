//! 对齐: `cn.hutool.core.date.DateUtil`
//!
//! # Timezone note
//! 无显式时区的日期字符串按 **UTC+08:00** 解析/格式化，对齐 Hutool
//! `DateUtilTest` 的 `TZ=Asia/Shanghai` 约定。

#![allow(dead_code, clippy::too_many_arguments)]

use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, NaiveTime, Timelike};

use crate::date::between_formatter::{BetweenFormatter, Level as BetweenLevel};
use crate::date::date_between::DateBetween;
use crate::date::date_field::DateField;
use crate::date::date_modifier::DateModifier;
use crate::date::date_pattern::{
    self, NORM_DATE_PATTERN, NORM_DATETIME_MS_PATTERN, NORM_DATETIME_PATTERN, NORM_TIME_PATTERN,
};
use crate::date::date_range::DateRange;
use crate::date::date_time::{DateTime, parity_zone};
use crate::date::date_unit::DateUnit;
use crate::date::local_date_time_util::LocalDateTimeUtil;
use crate::date::month::Month;
use crate::date::quarter::Quarter;
use crate::date::stop_watch::StopWatch;
use crate::date::time_interval::TimeInterval;
use crate::date::week::Week;
use crate::date::zodiac::Zodiac;
use crate::{CoreError, Result};

/// 对齐 Java: `cn.hutool.core.date.DateUtil`
#[derive(Debug, Clone, Default)]
pub struct DateUtil;

impl DateUtil {
    /// 兼容旧 pending_alignment。
    pub fn pending_alignment() -> &'static str {
        "chrono-backed DateUtil (Asia/Shanghai +08:00 parity zone)"
    }

    /// 当前 DateTime。
    pub fn date() -> DateTime {
        DateTime::now()
    }

    /// 从毫秒构造。
    pub fn date_from_millis(millis: i64) -> DateTime {
        DateTime::of_millis(millis)
    }

    /// 当前毫秒时间戳。
    pub fn current() -> i64 {
        chrono::Utc::now().timestamp_millis()
    }

    /// 当前秒时间戳。
    pub fn current_seconds() -> i64 {
        chrono::Utc::now().timestamp()
    }

    /// 当前时间字符串 `yyyy-MM-dd HH:mm:ss`。
    pub fn now() -> String {
        Self::format_datetime(Self::date())
    }

    /// 当前日期字符串 `yyyy-MM-dd`。
    pub fn today() -> String {
        Self::format_date(Self::date())
    }

    /// 本年。
    pub fn this_year() -> i32 {
        Self::date().year()
    }

    /// 本月（0-11）。
    pub fn this_month() -> i32 {
        Self::date().month()
    }

    // ---- field accessors ----

    /// 年份。
    pub fn year(date: DateTime) -> i32 {
        date.year()
    }
    /// 月份（1-12）。
    pub fn month(date: DateTime) -> i32 {
        date.month()
    }
    /// 月份枚举。
    pub fn month_enum(date: DateTime) -> Month {
        date.month_enum()
    }
    /// 季度（1-4）。
    pub fn quarter(date: DateTime) -> i32 {
        date.quarter()
    }
    /// 季度枚举。
    pub fn quarter_enum(date: DateTime) -> Quarter {
        date.quarter_enum()
    }
    /// 月内日。
    pub fn day_of_month(date: DateTime) -> i32 {
        date.day_of_month()
    }
    /// 年内日。
    pub fn day_of_year(date: DateTime) -> i32 {
        date.day_of_year()
    }
    /// 周内日（数字）。
    pub fn day_of_week(date: DateTime) -> i32 {
        date.day_of_week()
    }
    /// 星期枚举。
    pub fn day_of_week_enum(date: DateTime) -> Week {
        date.day_of_week_enum()
    }
    /// 是否周末。
    pub fn is_weekend(date: DateTime) -> bool {
        date.is_weekend()
    }
    /// 小时（`is_24` 为 false 时取 12 小时制）。
    pub fn hour(date: DateTime, is_24: bool) -> i32 {
        date.hour(is_24)
    }
    /// 分钟。
    pub fn minute(date: DateTime) -> i32 {
        date.minute()
    }
    /// 秒。
    pub fn second(date: DateTime) -> i32 {
        date.second()
    }
    /// 毫秒。
    pub fn millisecond(date: DateTime) -> i32 {
        date.millisecond()
    }
    /// 年内周数。
    pub fn week_of_year(date: DateTime) -> u32 {
        date.week_of_year()
    }
    /// 是否上午。
    pub fn is_am(date: DateTime) -> bool {
        date.hour(true) < 12
    }
    /// 是否下午。
    pub fn is_pm(date: DateTime) -> bool {
        !Self::is_am(date)
    }

    /// 年份与季度拼接（如 `20241`）。
    pub fn year_and_quarter(date: DateTime) -> String {
        format!("{}{}", date.year(), date.quarter())
    }

    /// 时间段内 yearAndQuarter 集合。
    pub fn year_and_quarter_range(start: DateTime, end: DateTime) -> Vec<String> {
        let mut out = Vec::new();
        let mut cur = Self::begin_of_month(start);
        let end_m = Self::begin_of_month(end);
        while cur <= end_m {
            let yq = Self::year_and_quarter(cur);
            if out.last() != Some(&yq) {
                out.push(yq);
            }
            cur = cur.offset(DateField::Month, 1);
        }
        out
    }

    // ---- format ----

    /// 按指定格式格式化日期。
    pub fn format(date: DateTime, pattern: &str) -> String {
        date.format(pattern)
    }
    /// 格式化日期时间 `yyyy-MM-dd HH:mm:ss`。
    pub fn format_datetime(date: DateTime) -> String {
        date.format(NORM_DATETIME_PATTERN)
    }
    /// 格式化日期 `yyyy-MM-dd`。
    pub fn format_date(date: DateTime) -> String {
        date.format(NORM_DATE_PATTERN)
    }
    /// 格式化时间 `HH:mm:ss`。
    pub fn format_time(date: DateTime) -> String {
        date.format(NORM_TIME_PATTERN)
    }
    /// 格式化 HTTP 日期。
    pub fn format_http_date(date: DateTime) -> String {
        date.format("EEE, dd MMM yyyy HH:mm:ss z")
    }

    /// 中文日期（小写数字 → 〇一二…）。
    pub fn format_chinese_date(date: DateTime, is_uppercase: bool, with_time: bool) -> String {
        let n = date.naive_local();
        let mut s = format!(
            "{}年{}月{}日",
            to_chinese_number(n.year() as u32, is_uppercase),
            to_chinese_number(n.month(), is_uppercase),
            to_chinese_number(n.day(), is_uppercase)
        );
        // Hutool uses 〇 for zero in year like 二〇一八
        if !is_uppercase {
            s = s.replace('零', "〇");
        }
        if with_time {
            s.push_str(&format!(
                "{}时{}分{}秒",
                to_chinese_number(n.hour(), is_uppercase),
                to_chinese_number(n.minute(), is_uppercase),
                to_chinese_number(n.second(), is_uppercase)
            ));
            if !is_uppercase {
                s = s.replace('零', "〇");
            }
        }
        s
    }

    // ---- parse ----

    /// 自动识别常见格式解析。
    pub fn parse(date_str: &str) -> Result<DateTime> {
        let s = date_str.trim();
        if s.is_empty() {
            return Err(CoreError::InvalidArgument {
                name: "dateStr",
                reason: "empty date string",
            });
        }
        if let Ok(dt) = Self::parse_with_format(s, NORM_DATETIME_MS_PATTERN) {
            return Ok(dt);
        }
        // try many patterns
        let patterns = [
            NORM_DATETIME_PATTERN,
            NORM_DATE_PATTERN,
            "yyyy/MM/dd",
            "yyyy/MM/dd HH:mm:ss",
            "yyyyMMdd",
            "yyyyMMddHHmmss",
            "yyyy-MM-dd HH:mm",
            "yyyy-MM-dd'T'HH:mm:ss",
            "yyyy-MM-dd'T'HH:mm:ss'Z'",
            "yyyy-MM-dd'T'HH:mm:ssXXX",
            "EEE MMM dd HH:mm:ss zzz yyyy", // JDK Date.toString
            "EEE, dd MMM yyyy HH:mm:ss z",  // HTTP / RFC
        ];
        for p in patterns {
            if let Ok(dt) = Self::parse_with_format(s, p) {
                return Ok(dt);
            }
        }
        // ISO / UTC variants
        if let Ok(dt) = Self::parse_utc(s) {
            return Ok(dt);
        }
        if let Ok(dt) = Self::parse_iso8601(s) {
            return Ok(dt);
        }
        // single number month/day: 2020-5-8
        if let Ok(dt) = parse_flexible_norm(s) {
            return Ok(dt);
        }
        Err(CoreError::InvalidArgument {
            name: "dateStr",
            reason: "unrecognized date format",
        })
    }

    /// 解析日期字符串为当天开始时刻。
    ///
    /// # Errors
    ///
    /// 无法识别格式时返回 [`CoreError::InvalidArgument`]。
    pub fn parse_date(date_str: &str) -> Result<DateTime> {
        Self::parse_with_format(date_str.trim(), NORM_DATE_PATTERN)
            .or_else(|_| Self::parse(date_str))
            .map(|d| Self::begin_of_day(d))
    }

    /// 解析日期时间字符串。
    ///
    /// # Errors
    ///
    /// 无法识别格式时返回 [`CoreError::InvalidArgument`]。
    pub fn parse_datetime(date_str: &str) -> Result<DateTime> {
        Self::parse_with_format(date_str.trim(), NORM_DATETIME_PATTERN)
            .or_else(|_| Self::parse(date_str))
    }

    /// 解析时间字符串为当日时刻。
    ///
    /// # Errors
    ///
    /// 格式不合法时返回解析错误。
    pub fn parse_time(time_str: &str) -> Result<DateTime> {
        let t = NaiveTime::parse_from_str(time_str.trim(), "%H:%M:%S").map_err(CoreError::from)?;
        let today = chrono::Utc::now()
            .with_timezone(&parity_zone())
            .date_naive();
        Ok(DateTime::of_naive(today.and_time(t)))
    }

    /// 按指定格式解析日期。
    ///
    /// # Errors
    ///
    /// 解析失败时返回 [`CoreError::InvalidArgument`]。
    pub fn parse_with_format(date_str: &str, format: &str) -> Result<DateTime> {
        let s = date_str.trim();
        if format == "#sss" {
            let secs: i64 = s.parse().map_err(|_| CoreError::InvalidArgument {
                name: "dateStr",
                reason: "invalid epoch seconds",
            })?;
            return Ok(DateTime::of_millis(secs * 1000));
        }
        if format == "#SSS" {
            let ms: i64 = s.parse().map_err(|_| CoreError::InvalidArgument {
                name: "dateStr",
                reason: "invalid epoch millis",
            })?;
            return Ok(DateTime::of_millis(ms));
        }
        // RFC / HTTP
        if format.contains("EEE") {
            return parse_http_like(s);
        }
        // Hutool SSS = milliseconds; chrono %f is nanoseconds — handle explicitly first.
        if format.contains('S') || s.contains('.') {
            if let Some((base, frac)) = s.rsplit_once('.') {
                if let Ok(ndt) = NaiveDateTime::parse_from_str(base, "%Y-%m-%d %H:%M:%S") {
                    let mut frac_s: String = frac.chars().filter(|c| c.is_ascii_digit()).collect();
                    while frac_s.len() < 3 {
                        frac_s.push('0');
                    }
                    let ms: u32 = frac_s
                        .chars()
                        .take(3)
                        .collect::<String>()
                        .parse()
                        .unwrap_or(0);
                    let ndt = ndt.with_nanosecond(ms * 1_000_000).unwrap_or(ndt);
                    return Ok(DateTime::of_naive(ndt));
                }
            }
        }
        let chrono_pat = date_pattern::to_chrono_format(format);
        if let Ok(ndt) = NaiveDateTime::parse_from_str(s, &chrono_pat) {
            return Ok(DateTime::of_naive(ndt));
        }
        if let Ok(nd) = NaiveDate::parse_from_str(s, &chrono_pat) {
            return Ok(DateTime::of_naive(nd.and_hms_opt(0, 0, 0).unwrap()));
        }
        if let Ok(nd) = NaiveDate::parse_from_str(s, "%Y-%m-%d") {
            return Ok(DateTime::of_naive(nd.and_hms_opt(0, 0, 0).unwrap()));
        }
        // pure digits yyyyMMdd / yyyyMMddHHmmss / with millis
        if s.chars().all(|c| c.is_ascii_digit()) {
            return parse_pure_digits(s);
        }
        Err(CoreError::InvalidArgument {
            name: "dateStr",
            reason: "parse failed for format",
        })
    }

    /// 解析 UTC 字符串。
    ///
    /// # Errors
    ///
    /// 格式不合法时返回 [`CoreError::InvalidArgument`]。
    pub fn parse_utc(utc: &str) -> Result<DateTime> {
        let s = utc.trim().replace('Z', "+00:00");
        // 2019-09-17T13:26:17.000Z or with offset
        if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&s) {
            return Ok(DateTime::of_millis(dt.timestamp_millis()));
        }
        // without colon in offset +0800
        let normalized = normalize_offset(&s);
        if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&normalized) {
            return Ok(DateTime::of_millis(dt.timestamp_millis()));
        }
        // space instead of T
        let t = s.replace(' ', "T");
        if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&t) {
            return Ok(DateTime::of_millis(dt.timestamp_millis()));
        }
        Err(CoreError::InvalidArgument {
            name: "utcString",
            reason: "invalid UTC date",
        })
    }

    /// 解析 ISO8601 字符串。
    ///
    /// # Errors
    ///
    /// 格式不合法时返回 [`CoreError::InvalidArgument`]。
    pub fn parse_iso8601(iso: &str) -> Result<DateTime> {
        Self::parse_utc(iso).or_else(|_| {
            let s = iso.trim();
            if let Ok(ndt) = NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S") {
                return Ok(DateTime::of_naive(ndt));
            }
            if let Ok(ndt) = NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.f") {
                return Ok(DateTime::of_naive(ndt));
            }
            Err(CoreError::InvalidArgument {
                name: "iso8601String",
                reason: "invalid ISO8601 date",
            })
        })
    }

    /// 解析 RFC2822 字符串。
    ///
    /// # Errors
    ///
    /// 格式不合法时返回解析错误。
    pub fn parse_rfc2822(source: &str) -> Result<DateTime> {
        parse_http_like(source.trim())
    }

    /// 解析 CST 字符串。
    ///
    /// # Errors
    ///
    /// 格式不合法时返回解析错误。
    pub fn parse_cst(cst: &str) -> Result<DateTime> {
        // CST often means China Standard Time in Hutool context when parsing JDK strings
        parse_http_like(cst.trim()).or_else(|_| Self::parse(cst))
    }

    // ---- begin / end ----

    /// 当天开始时刻（00:00:00）。
    pub fn begin_of_day(date: DateTime) -> DateTime {
        DateModifier::truncate(date, DateField::DayOfMonth)
    }
    /// 当天结束时刻（23:59:59）。
    pub fn end_of_day(date: DateTime) -> DateTime {
        // Hutool endOfDay defaults to 23:59:59.000 (ms truncated in toString)
        DateModifier::ceiling(date, DateField::DayOfMonth, true)
    }
    /// 小时开始时刻。
    pub fn begin_of_hour(date: DateTime) -> DateTime {
        DateModifier::truncate(date, DateField::HourOfDay)
    }
    /// 小时结束时刻。
    pub fn end_of_hour(date: DateTime) -> DateTime {
        DateModifier::ceiling(date, DateField::HourOfDay, true)
    }
    /// 分钟开始时刻。
    pub fn begin_of_minute(date: DateTime) -> DateTime {
        DateModifier::truncate(date, DateField::Minute)
    }
    /// 分钟结束时刻。
    pub fn end_of_minute(date: DateTime) -> DateTime {
        DateModifier::ceiling(date, DateField::Minute, true)
    }
    /// 秒开始时刻。
    pub fn begin_of_second(date: DateTime) -> DateTime {
        DateModifier::truncate(date, DateField::Second)
    }
    /// 秒结束时刻。
    pub fn end_of_second(date: DateTime) -> DateTime {
        DateModifier::ceiling(date, DateField::Second, true)
    }
    /// 月初。
    pub fn begin_of_month(date: DateTime) -> DateTime {
        DateModifier::truncate(date, DateField::Month)
    }
    /// 月末。
    pub fn end_of_month(date: DateTime) -> DateTime {
        DateModifier::ceiling(date, DateField::Month, true)
    }
    /// 年初。
    pub fn begin_of_year(date: DateTime) -> DateTime {
        DateModifier::truncate(date, DateField::Year)
    }
    /// 年末。
    pub fn end_of_year(date: DateTime) -> DateTime {
        DateModifier::ceiling(date, DateField::Year, true)
    }
    /// 季初。
    pub fn begin_of_quarter(date: DateTime) -> DateTime {
        let q = date.quarter();
        let month = (q - 1) * 3 + 1;
        DateTime::of_ymd_hms(date.year(), month as u32, 1, 0, 0, 0).unwrap_or(date)
    }
    /// 季末。
    pub fn end_of_quarter(date: DateTime) -> DateTime {
        let q = date.quarter_enum();
        let (m, d) = q.last_month_day();
        let mut dt = DateTime::of_ymd_hms(date.year(), m, d, 23, 59, 59).unwrap_or(date);
        // keep
        let _ = &mut dt;
        dt
    }

    /// 一周开始（使用 date 的 firstDayOfWeek，默认周一）。
    pub fn begin_of_week(date: DateTime) -> DateTime {
        Self::begin_of_week_with(date, true)
    }

    /// `is_monday_as_first`：true 从周一开始。
    pub fn begin_of_week_with(date: DateTime, is_monday_as_first: bool) -> DateTime {
        let first = if is_monday_as_first {
            Week::Monday
        } else {
            Week::Sunday
        };
        let naive = date.naive_local().date();
        let wd = naive.weekday();
        let from_first = if is_monday_as_first {
            wd.num_days_from_monday()
        } else {
            wd.num_days_from_sunday()
        };
        let begin = naive - Duration::days(from_first as i64);
        let mut dt = DateTime::of_naive(begin.and_hms_opt(0, 0, 0).unwrap());
        dt.set_first_day_of_week(first);
        dt
    }

    /// 周结束（默认周一为一周开始）。
    pub fn end_of_week(date: DateTime) -> DateTime {
        Self::end_of_week_with(date, true)
    }

    /// `is_monday_as_first`：true 从周一开始的周结束。
    pub fn end_of_week_with(date: DateTime, is_monday_as_first: bool) -> DateTime {
        let begin = Self::begin_of_week_with(date, is_monday_as_first);
        let end = begin.naive_local().date() + Duration::days(6);
        DateTime::of_naive(end.and_hms_opt(23, 59, 59).unwrap())
    }

    /// 截断到指定字段。
    pub fn truncate(date: DateTime, field: DateField) -> DateTime {
        DateModifier::truncate(date, field)
    }
    /// 向上取整到指定字段。
    pub fn ceiling(date: DateTime, field: DateField) -> DateTime {
        DateModifier::ceiling(date, field, false)
    }
    /// 向上取整，可选截断毫秒。
    pub fn ceiling_ms(date: DateTime, field: DateField, truncate_ms: bool) -> DateTime {
        DateModifier::ceiling(date, field, truncate_ms)
    }
    /// 四舍五入到指定字段。
    pub fn round(date: DateTime, field: DateField) -> DateTime {
        DateModifier::round(date, field)
    }

    // ---- offset ----

    /// 按字段偏移。
    pub fn offset(date: DateTime, field: DateField, offset: i64) -> DateTime {
        date.offset(field, offset)
    }
    /// 按天偏移。
    pub fn offset_day(date: DateTime, offset: i64) -> DateTime {
        date.offset(DateField::DayOfMonth, offset)
    }
    /// 按小时偏移。
    pub fn offset_hour(date: DateTime, offset: i64) -> DateTime {
        date.offset(DateField::HourOfDay, offset)
    }
    /// 按分钟偏移。
    pub fn offset_minute(date: DateTime, offset: i64) -> DateTime {
        date.offset(DateField::Minute, offset)
    }
    /// 按月偏移。
    pub fn offset_month(date: DateTime, offset: i64) -> DateTime {
        date.offset(DateField::Month, offset)
    }
    /// 按周偏移。
    pub fn offset_week(date: DateTime, offset: i64) -> DateTime {
        date.offset(DateField::WeekOfYear, offset)
    }

    // ---- between ----

    /// 计算两时间在指定单位上的间隔。
    pub fn between(begin: DateTime, end: DateTime, unit: DateUnit) -> i64 {
        DateBetween::new(begin, end, true).between(unit)
    }
    /// 间隔毫秒数。
    pub fn between_ms(begin: DateTime, end: DateTime) -> i64 {
        (end.get_time() - begin.get_time()).abs()
    }
    /// 间隔天数。
    pub fn between_day(begin: DateTime, end: DateTime, is_reset: bool) -> i64 {
        DateBetween::new(begin, end, true).between_day(is_reset)
    }
    /// 间隔周数。
    pub fn between_week(begin: DateTime, end: DateTime, is_reset: bool) -> i64 {
        DateBetween::new(begin, end, true).between_week(is_reset)
    }
    /// 间隔月数。
    pub fn between_month(begin: DateTime, end: DateTime, is_reset: bool) -> i64 {
        DateBetween::new(begin, end, true).between_month(is_reset)
    }
    /// 间隔年数。
    pub fn between_year(begin: DateTime, end: DateTime, is_reset: bool) -> i64 {
        DateBetween::new(begin, end, true).between_year(is_reset)
    }

    /// 格式化两时间间隔。
    pub fn format_between(begin: DateTime, end: DateTime) -> String {
        BetweenFormatter::new(Self::between_ms(begin, end), BetweenLevel::Millisecond, 3).format()
    }
    /// 按指定级别格式化间隔。
    pub fn format_between_level(begin: DateTime, end: DateTime, level: BetweenLevel) -> String {
        BetweenFormatter::new(Self::between_ms(begin, end), level, 1).format()
    }
    /// 格式化毫秒间隔。
    pub fn format_between_ms(between_ms: i64, level: BetweenLevel) -> String {
        BetweenFormatter::new(between_ms, level, 3).format()
    }

    // ---- compare / same ----

    /// 比较两时间（d1 < d2 为负，相等为 0）。
    pub fn compare(d1: DateTime, d2: DateTime) -> i32 {
        d1.get_time().cmp(&d2.get_time()) as i32
    }
    /// 是否同一天。
    pub fn is_same_day(d1: DateTime, d2: DateTime) -> bool {
        d1.naive_local().date() == d2.naive_local().date()
    }
    /// 是否同一月。
    pub fn is_same_month(d1: DateTime, d2: DateTime) -> bool {
        let a = d1.naive_local();
        let b = d2.naive_local();
        a.year() == b.year() && a.month() == b.month()
    }
    /// 是否同一周。
    pub fn is_same_week(d1: DateTime, d2: DateTime, is_monday_as_first: bool) -> bool {
        let b1 = Self::begin_of_week_with(d1, is_monday_as_first);
        let b2 = Self::begin_of_week_with(d2, is_monday_as_first);
        b1.naive_local().date() == b2.naive_local().date()
    }
    /// 是否在区间内（含端点）。
    pub fn is_in(date: DateTime, begin: DateTime, end: DateTime) -> bool {
        date >= begin && date <= end
    }
    /// 两区间是否重叠。
    pub fn is_overlap(start1: DateTime, end1: DateTime, start2: DateTime, end2: DateTime) -> bool {
        start1 <= end2 && start2 <= end1
    }
    /// 是否已过期。
    pub fn is_expired(start: DateTime, end: DateTime) -> bool {
        end < start
    }
    /// 是否当月最后一天。
    pub fn is_last_day_of_month(date: DateTime) -> bool {
        let n = date.naive_local();
        let tomorrow = n.date() + Duration::days(1);
        tomorrow.month() != n.month()
    }

    // ---- age / time ----

    /// 计算年龄（周岁）。
    pub fn age(birthday: DateTime, date_to_compare: DateTime) -> i32 {
        let b = birthday.naive_local().date();
        let c = date_to_compare.naive_local().date();
        let mut age = c.year() - b.year();
        if (c.month(), c.day()) < (b.month(), b.day()) {
            age -= 1;
        }
        age.max(0)
    }

    /// 时间字符串 `HH:mm:ss` 转为当日秒数。
    pub fn time_to_second(time: &str) -> i64 {
        let t = NaiveTime::parse_from_str(time, "%H:%M:%S").expect("HH:mm:ss");
        i64::from(t.num_seconds_from_midnight())
    }

    /// 秒数转为 `HH:mm:ss` 字符串。
    pub fn second_to_time(seconds: i64) -> String {
        let h = seconds / 3600;
        let m = (seconds % 3600) / 60;
        let s = seconds % 60;
        format!("{h:02}:{m:02}:{s:02}")
    }

    /// 转为 UTC 时间。
    pub fn to_instant(date: DateTime) -> chrono::DateTime<chrono::Utc> {
        chrono::DateTime::from_timestamp_millis(date.get_time())
            .unwrap_or(chrono::DateTime::UNIX_EPOCH)
    }

    /// 月份范围列表（每月最后一天等场景用 end of month）。
    pub fn range_to_list(start: DateTime, end: DateTime, unit: DateField) -> Vec<DateTime> {
        let mut out = Vec::new();
        let mut cur = start;
        while cur <= end {
            out.push(cur);
            cur = cur.offset(unit, 1);
            if out.len() > 10_000 {
                break;
            }
        }
        out
    }

    /// 对齐 Java: `DateUtil.dateSecond()`（当前时间截断到秒）
    pub fn date_second() -> DateTime {
        Self::begin_of_second(Self::date())
    }

    /// 对齐 Java: `DateUtil.yesterday()`
    pub fn yesterday() -> DateTime {
        Self::offset_day(Self::date(), -1)
    }

    /// 对齐 Java: `DateUtil.tomorrow()`
    pub fn tomorrow() -> DateTime {
        Self::offset_day(Self::date(), 1)
    }

    /// 对齐 Java: `DateUtil.lastWeek()`
    pub fn last_week() -> DateTime {
        Self::offset_week(Self::date(), -1)
    }

    /// 对齐 Java: `DateUtil.nextWeek()`
    pub fn next_week() -> DateTime {
        Self::offset_week(Self::date(), 1)
    }

    /// 对齐 Java: `DateUtil.lastMonth()`
    pub fn last_month() -> DateTime {
        Self::offset_month(Self::date(), -1)
    }

    /// 对齐 Java: `DateUtil.nextMonth()`
    pub fn next_month() -> DateTime {
        Self::offset_month(Self::date(), 1)
    }

    /// 对齐 Java: `DateUtil.offsetMillisecond(Date, int)`
    pub fn offset_millisecond(date: DateTime, offset: i64) -> DateTime {
        date.offset(DateField::Millisecond, offset)
    }

    /// 对齐 Java: `DateUtil.offsetSecond(Date, int)`
    pub fn offset_second(date: DateTime, offset: i64) -> DateTime {
        date.offset(DateField::Second, offset)
    }

    /// 对齐 Java: `DateUtil.offsetYear(Date, int)`
    pub fn offset_year(date: DateTime, offset: i64) -> DateTime {
        date.offset(DateField::Year, offset)
    }

    /// 对齐 Java: `DateUtil.createStopWatch()`
    pub fn create_stop_watch() -> StopWatch {
        StopWatch::new()
    }

    /// 对齐 Java: `DateUtil.createStopWatch(String)`
    pub fn create_stop_watch_id(id: impl Into<String>) -> StopWatch {
        StopWatch::with_id(id)
    }

    /// 对齐 Java: `DateUtil.timer()`
    pub fn timer() -> TimeInterval {
        TimeInterval::new()
    }

    /// 对齐 Java: `DateUtil.timer(boolean)`
    pub fn timer_nano(is_nano: bool) -> TimeInterval {
        TimeInterval::with_nano(is_nano)
    }

    /// 对齐 Java: `DateUtil.range(Date, Date, DateField)`
    pub fn range(start: DateTime, end: DateTime, unit: DateField) -> DateRange {
        DateRange::new(start, end, unit)
    }

    /// 对齐 Java: `DateUtil.rangeFunc`
    pub fn range_func<T, F>(start: DateTime, end: DateTime, unit: DateField, mut func: F) -> Vec<T>
    where
        F: FnMut(DateTime) -> T,
    {
        Self::range_to_list(start, end, unit)
            .into_iter()
            .map(|d| func(d))
            .collect()
    }

    /// 对齐 Java: `DateUtil.rangeConsume`
    pub fn range_consume<F>(start: DateTime, end: DateTime, unit: DateField, mut consumer: F)
    where
        F: FnMut(DateTime),
    {
        for d in Self::range_to_list(start, end, unit) {
            consumer(d);
        }
    }

    /// 对齐 Java: `DateUtil.rangeContains`
    pub fn range_contains(a: &DateRange, b: &DateRange) -> Vec<DateTime> {
        let list_b = b.to_list();
        a.to_list()
            .into_iter()
            .filter(|d| list_b.iter().any(|x| x == d))
            .collect()
    }

    /// 对齐 Java: `DateUtil.rangeNotContains`
    pub fn range_not_contains(a: &DateRange, b: &DateRange) -> Vec<DateTime> {
        let list_b = b.to_list();
        a.to_list()
            .into_iter()
            .filter(|d| !list_b.iter().any(|x| x == d))
            .collect()
    }

    /// 对齐 Java: `DateUtil.spendNt(long)` — 相对 `System.nanoTime()` 风格差值。
    pub fn spend_nt(pre_time: i64) -> i64 {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as i64)
            .unwrap_or(0);
        now - pre_time
    }

    /// 对齐 Java: `DateUtil.spendMs(long)`
    pub fn spend_ms(pre_time: i64) -> i64 {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as i64)
            .unwrap_or(0);
        now - pre_time
    }

    /// 对齐 Java: `DateUtil.toIntSecond(Date)`
    pub fn to_int_second(date: DateTime) -> i32 {
        (date.get_time() / 1000) as i32
    }

    /// 对齐 Java: `DateUtil.nanosToMillis(long)`
    pub fn nanos_to_millis(duration: i64) -> i64 {
        duration / 1_000_000
    }

    /// 对齐 Java: `DateUtil.nanosToSeconds(long)`
    pub fn nanos_to_seconds(duration: i64) -> f64 {
        duration as f64 / 1_000_000_000.0
    }

    /// 对齐 Java: `DateUtil.getZodiac(int, int)` — month 从 1 开始。
    pub fn get_zodiac(month: i32, day: i32) -> Option<&'static str> {
        Zodiac::get_zodiac(month.saturating_sub(1), day)
    }

    /// 对齐 Java: `DateUtil.getChineseZodiac(int)`
    pub fn get_chinese_zodiac(year: i32) -> Option<&'static str> {
        Zodiac::get_chinese_zodiac(year)
    }

    /// 对齐 Java: `DateUtil.formatLocalDateTime`
    pub fn format_local_date_time(dt: chrono::NaiveDateTime) -> String {
        LocalDateTimeUtil::format_normal(dt)
    }

    /// 对齐 Java: `DateUtil.parseLocalDateTime(CharSequence)`
    pub fn parse_local_date_time(date_str: &str) -> Result<chrono::NaiveDateTime> {
        LocalDateTimeUtil::parse(date_str)
    }

    /// 对齐 Java: `DateUtil.parseLocalDateTime(CharSequence, String)`
    pub fn parse_local_date_time_format(
        date_str: &str,
        format: &str,
    ) -> Result<chrono::NaiveDateTime> {
        LocalDateTimeUtil::parse_format(date_str, format)
    }

    /// 对齐 Java: `DateUtil.thisHour(boolean)`
    pub fn this_hour(is_24: bool) -> i32 {
        Self::hour(Self::date(), is_24)
    }

    /// 对齐 Java: `DateUtil.thisMinute()`
    pub fn this_minute() -> i32 {
        Self::minute(Self::date())
    }

    /// 对齐 Java: `DateUtil.thisSecond()`
    pub fn this_second() -> i32 {
        Self::second(Self::date())
    }

    /// 对齐 Java: `DateUtil.thisMillisecond()`
    pub fn this_millisecond() -> i32 {
        Self::millisecond(Self::date())
    }

    /// 对齐 Java: `DateUtil.thisDayOfMonth()`
    pub fn this_day_of_month() -> i32 {
        Self::day_of_month(Self::date())
    }

    /// 对齐 Java: `DateUtil.thisDayOfWeek()`
    pub fn this_day_of_week() -> i32 {
        Self::day_of_week(Self::date())
    }

    /// 对齐 Java: `DateUtil.thisDayOfWeekEnum()`
    pub fn this_day_of_week_enum() -> Week {
        Self::day_of_week_enum(Self::date())
    }

    /// 对齐 Java: `DateUtil.thisMonthEnum()`
    pub fn this_month_enum() -> Month {
        Self::month_enum(Self::date())
    }

    /// 对齐 Java: `DateUtil.thisWeekOfYear()`
    pub fn this_week_of_year() -> u32 {
        Self::week_of_year(Self::date())
    }

    /// 对齐 Java: `DateUtil.weekOfMonth(Date)`
    pub fn week_of_month(date: DateTime) -> u32 {
        date.week_of_month()
    }

    /// 对齐 Java: `DateUtil.thisWeekOfMonth()`
    pub fn this_week_of_month() -> u32 {
        Self::week_of_month(Self::date())
    }

    /// 对齐 Java: `DateUtil.isLeapYear(int)`
    pub fn is_leap_year(year: i32) -> bool {
        NaiveDate::from_ymd_opt(year, 2, 29).is_some()
    }

    /// 对齐 Java: `DateUtil.isSameTime(Date, Date)`（毫秒相等）
    pub fn is_same_time(d1: DateTime, d2: DateTime) -> bool {
        d1.get_time() == d2.get_time()
    }

    /// 对齐 Java: `DateUtil.ageOfNow(Date)`
    pub fn age_of_now(birthday: DateTime) -> i32 {
        Self::age(birthday, Self::date())
    }

    /// 对齐 Java: `DateUtil.lengthOfMonth(int, boolean)`（month 1-12）
    pub fn length_of_month(month: u32, is_leap_year: bool) -> u32 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if is_leap_year {
                    29
                } else {
                    28
                }
            }
            _ => 0,
        }
    }
}

fn to_chinese_number(n: u32, uppercase: bool) -> String {
    // Hutool NumberChineseFormatter for date uses 〇一二三四五六七八九
    const DIGITS: &[&str] = &["〇", "一", "二", "三", "四", "五", "六", "七", "八", "九"];
    const DIGITS_UP: &[&str] = &["零", "壹", "贰", "叁", "肆", "伍", "陆", "柒", "捌", "玖"];
    let table = if uppercase { DIGITS_UP } else { DIGITS };
    if n == 0 {
        return table[0].to_string();
    }
    // special for hour/minute like 十二、零
    if n < 10 {
        return table[n as usize].to_string();
    }
    if n < 20 {
        if n == 10 {
            return "十".to_string();
        }
        return format!("十{}", table[(n % 10) as usize]);
    }
    if n < 100 {
        let tens = n / 10;
        let ones = n % 10;
        if ones == 0 {
            return format!("{}十", table[tens as usize]);
        }
        return format!("{}十{}", table[tens as usize], table[ones as usize]);
    }
    // year etc: digit by digit
    n.to_string()
        .chars()
        .map(|c| table[(c as u8 - b'0') as usize])
        .collect()
}

fn parse_pure_digits(s: &str) -> Result<DateTime> {
    match s.len() {
        8 => {
            let y: i32 = s[0..4].parse().unwrap();
            let m: u32 = s[4..6].parse().unwrap();
            let d: u32 = s[6..8].parse().unwrap();
            DateTime::of_ymd_hms(y, m, d, 0, 0, 0)
        }
        14 => {
            let y: i32 = s[0..4].parse().unwrap();
            let m: u32 = s[4..6].parse().unwrap();
            let d: u32 = s[6..8].parse().unwrap();
            let h: u32 = s[8..10].parse().unwrap();
            let mi: u32 = s[10..12].parse().unwrap();
            let sec: u32 = s[12..14].parse().unwrap();
            DateTime::of_ymd_hms(y, m, d, h, mi, sec)
        }
        17 => {
            let base = parse_pure_digits(&s[..14])?;
            let ms: u32 = s[14..17].parse().unwrap_or(0);
            let n = base.naive_local().with_nanosecond(ms * 1_000_000).unwrap();
            Ok(DateTime::of_naive(n))
        }
        _ => Err(CoreError::InvalidArgument {
            name: "dateStr",
            reason: "unsupported pure digit length",
        }),
    }
}

fn parse_flexible_norm(s: &str) -> Result<DateTime> {
    // yyyy-M-d[ H:m[:s[.S]]]
    let re = regex::Regex::new(
        r"^(\d{4})-(\d{1,2})-(\d{1,2})(?:\s+(\d{1,2}):(\d{1,2})(?::(\d{1,2})(?:\.(\d{1,6}))?)?)?$",
    )
    .unwrap();
    let caps = re.captures(s).ok_or(CoreError::InvalidArgument {
        name: "dateStr",
        reason: "flexible parse failed",
    })?;
    let y: i32 = caps[1].parse().unwrap();
    let m: u32 = caps[2].parse().unwrap();
    let d: u32 = caps[3].parse().unwrap();
    let h: u32 = caps
        .get(4)
        .map(|c| c.as_str().parse().unwrap())
        .unwrap_or(0);
    let mi: u32 = caps
        .get(5)
        .map(|c| c.as_str().parse().unwrap())
        .unwrap_or(0);
    let sec: u32 = caps
        .get(6)
        .map(|c| c.as_str().parse().unwrap())
        .unwrap_or(0);
    let mut dt = DateTime::of_ymd_hms(y, m, d, h, mi, sec)?;
    if let Some(frac) = caps.get(7) {
        let mut frac_s = frac.as_str().to_string();
        while frac_s.len() < 3 {
            frac_s.push('0');
        }
        let ms: u32 = frac_s
            .chars()
            .take(3)
            .collect::<String>()
            .parse()
            .unwrap_or(0);
        let n = dt.naive_local().with_nanosecond(ms * 1_000_000).unwrap();
        dt = DateTime::of_naive(n);
    }
    Ok(dt)
}

fn normalize_offset(s: &str) -> String {
    // 2020-01-01T00:00:00+0800 -> +08:00
    if s.len() >= 5 {
        let bytes = s.as_bytes();
        if (bytes[bytes.len() - 5] == b'+' || bytes[bytes.len() - 5] == b'-')
            && bytes[bytes.len() - 4].is_ascii_digit()
        {
            let (head, off) = s.split_at(s.len() - 5);
            return format!("{head}{}{}:{}", &off[..1], &off[1..3], &off[3..]);
        }
    }
    s.to_string()
}

fn parse_http_like(s: &str) -> Result<DateTime> {
    // Try RFC2822
    if let Ok(dt) = chrono::DateTime::parse_from_rfc2822(s) {
        return Ok(DateTime::of_millis(dt.timestamp_millis()));
    }
    // JDK Date.toString: "EEE MMM dd HH:mm:ss zzz yyyy" e.g. Thu Mar 01 00:00:00 CST 2018
    let formats = [
        "%a %b %d %H:%M:%S %Z %Y",
        "%a %b %d %H:%M:%S CST %Y",
        "%a, %d %b %Y %H:%M:%S GMT",
        "%a, %d %b %Y %H:%M:%S %z",
    ];
    for f in formats {
        if let Ok(ndt) = NaiveDateTime::parse_from_str(s, f) {
            // CST as +08 for Hutool China context
            return Ok(DateTime::of_naive(ndt));
        }
    }
    // Manual tokenize JDK format
    let parts: Vec<&str> = s.split_whitespace().collect();
    if parts.len() >= 6 {
        // EEE MMM dd HH:mm:ss zzz yyyy
        let mon = month_abbr(parts[1]).ok_or(CoreError::InvalidArgument {
            name: "dateStr",
            reason: "bad month",
        })?;
        let day: u32 = parts[2].parse().unwrap_or(1);
        let time = NaiveTime::parse_from_str(parts[3], "%H:%M:%S").map_err(CoreError::from)?;
        let year: i32 = parts[parts.len() - 1].parse().unwrap_or(1970);
        let nd = NaiveDate::from_ymd_opt(year, mon, day).ok_or(CoreError::DateOverflow)?;
        return Ok(DateTime::of_naive(nd.and_time(time)));
    }
    Err(CoreError::InvalidArgument {
        name: "dateStr",
        reason: "http/jdk parse failed",
    })
}

fn month_abbr(s: &str) -> Option<u32> {
    let m = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    m.iter()
        .position(|x| x.eq_ignore_ascii_case(s))
        .map(|i| i as u32 + 1)
}
