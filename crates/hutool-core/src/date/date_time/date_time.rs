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

/// 对齐 Java: `cn.hutool.core.date.DateTime`
#[derive(Debug, Clone, Copy)]
pub struct DateTime {
    /// epoch 毫秒
    pub(crate) millis: i64,
    /// 一周起始（默认周一，便于 beginOfWeek 与常见中国习惯一致；可 set）
    pub(crate) first_day_of_week: Week,
    /// 可变模式（Hutool mutable）
    pub(crate) mutable: bool,
}

use super::{between_unit, days_in_month, format_with_pattern, parity_zone, week_of_year_mon_min1};
