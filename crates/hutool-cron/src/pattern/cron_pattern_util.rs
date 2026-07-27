#![allow(clippy::missing_panics_doc)]
//! 对齐: `cn.hutool.cron.pattern.CronPatternUtil`
//! 来源: hutool-cron/src/main/java/cn/hutool/cron/pattern/CronPatternUtil.java
//! 中文说明: Cron 表达式匹配日期计算的辅助工具类。

use std::{fmt, str::FromStr};

use chrono::{DateTime, Datelike, Duration as ChronoDuration, TimeZone, Timelike, Utc};
use cron::Schedule;

use crate::CronError;

use super::cron_pattern::CronPattern;

/// 对齐: `cn.hutool.cron.pattern.CronPatternUtil`
/// 中文说明: Cron 表达式匹配日期计算的辅助工具类。
///
/// Helpers for calculating matching dates with explicit bounds.
pub struct CronPatternUtil;

impl CronPatternUtil {
    /// 中文说明: 返回 `start` 之后的下一个匹配日期（秒级匹配）。
    /// 对齐 Java 方法: `nextDateAfter`
    #[must_use]
    pub fn next_date_after(pattern: &CronPattern, start: DateTime<Utc>) -> Option<DateTime<Utc>> {
        pattern.next_match_after(start, true)
    }

    /// 中文说明: 返回 `start` 之后的下一个匹配日期（可指定精度）。
    /// 对齐 Java 方法: `nextDateAfter`
    #[must_use]
    pub fn next_date_after_with_precision(
        pattern: &CronPattern,
        start: DateTime<Utc>,
        match_second: bool,
    ) -> Option<DateTime<Utc>> {
        pattern.next_match_after(start, match_second)
    }

    /// 中文说明: 返回时间窗口内最多 `count` 个匹配日期。
    /// 对齐 Java 方法: `matchedDates`
    pub fn matched_dates(
        pattern: &CronPattern,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        count: usize,
        match_second: bool,
    ) -> Result<Vec<DateTime<Utc>>, CronError> {
        if end < start {
            return Err(CronError::InvalidDateRange);
        }
        let mut result = Vec::with_capacity(count.min(64));
        let mut cursor = start - ChronoDuration::seconds(1);
        while result.len() < count {
            let Some(next) = pattern.next_match_after(cursor, match_second) else {
                break;
            };
            if next > end {
                break;
            }
            result.push(next);
            cursor = next;
        }
        Ok(result)
    }

    /// 中文说明: Hutool `matchedDates(pattern, start, count, matchSecond)` —— 结束时间默认为年末。
    /// 对齐 Java 方法: `matchedDates`
    pub fn matched_dates_count(
        pattern: &str,
        start: DateTime<Utc>,
        count: usize,
        match_second: bool,
    ) -> Result<Vec<DateTime<Utc>>, CronError> {
        let parsed = CronPattern::parse(pattern)?;
        let end = end_of_year(start);
        Self::matched_dates(&parsed, start, end, count, match_second)
    }

    /// 中文说明: Hutool `matchedDates(pattern, start, end, count, matchSecond)`。
    /// 对齐 Java 方法: `matchedDates`
    pub fn matched_dates_str(
        pattern: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        count: usize,
        match_second: bool,
    ) -> Result<Vec<DateTime<Utc>>, CronError> {
        let parsed = CronPattern::parse(pattern)?;
        Self::matched_dates(&parsed, start, end, count, match_second)
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
