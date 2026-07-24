//! 对齐: `cn.hutool.core.date.DateTime`
//!
//! # Timezone note
//! 默认按 UTC+08:00（Asia/Shanghai 无夏令时偏移）解释/格式化墙钟时间，
//! 与 Hutool `DateUtilTest` 的 `TZ=Asia/Shanghai` 约定一致。

#![allow(dead_code)]

use chrono::{Datelike, Duration, FixedOffset, NaiveDate, NaiveDateTime, Timelike, Weekday};

use crate::date::date_field::DateField;
use crate::date::date_pattern::{self, NORM_DATETIME_MS_PATTERN, NORM_DATETIME_PATTERN};
use crate::date::date_unit::DateUnit;
use crate::date::month::Month;
use crate::date::quarter::Quarter;
use crate::date::week::Week;
use crate::{CoreError, Result};

mod date_time;
mod hutool_date_time;

pub use date_time::DateTime;
pub use hutool_date_time::HutoolDateTime;

pub fn parity_zone() -> FixedOffset {
    FixedOffset::east_opt(8 * 3600).expect("fixed +08:00")
}

pub fn week_of_year_mon_min1(date: NaiveDate) -> u32 {
    let jan1 = NaiveDate::from_ymd_opt(date.year(), 1, 1).unwrap();
    let jan1_from_mon = jan1.weekday().num_days_from_monday();
    let day_of_year = date.ordinal();
    ((day_of_year + jan1_from_mon - 1) / 7) + 1
}

fn days_in_month(year: i32, month: u32) -> u32 {
    NaiveDate::from_ymd_opt(year, month + 1, 1)
        .unwrap_or_else(|| NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap())
        .pred_opt()
        .unwrap()
        .day()
}

pub fn format_with_pattern(dt: DateTime, pattern: &str) -> String {
    if pattern == "#sss" {
        return (dt.get_time() / 1000).to_string();
    }
    if pattern == "#SSS" {
        return dt.get_time().to_string();
    }
    let naive = dt.naive_local();
    // Millisecond pattern
    if pattern.contains("SSS") || pattern == NORM_DATETIME_MS_PATTERN {
        let base = naive.format("%Y-%m-%d %H:%M:%S").to_string();
        let ms = naive.nanosecond() / 1_000_000;
        return format!("{base}.{ms:03}");
    }
    // HTTP date in GMT
    if pattern.contains("EEE") && pattern.contains("MMM") {
        let utc = chrono::DateTime::from_timestamp_millis(dt.get_time())
            .unwrap_or(chrono::DateTime::UNIX_EPOCH)
            .naive_utc();
        // Wed, 02 Jan 2019 14:32:01 GMT
        return utc.format("%a, %d %b %Y %H:%M:%S GMT").to_string();
    }
    let chrono_pat = date_pattern::to_chrono_format(pattern);
    // Handle single-digit month/day patterns that chrono always zero-pads: post-process if needed
    naive.format(&chrono_pat).to_string()
}

pub fn between_unit(begin: DateTime, end: DateTime, unit: DateUnit) -> i64 {
    (end.get_time() - begin.get_time()) / unit.get_millis()
}
