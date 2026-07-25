#![allow(clippy::missing_panics_doc)]
//! Hutool-aligned cron patterns, builders, parsers, and matchers.


use std::{fmt, str::FromStr};

use chrono::{DateTime, Datelike, Duration as ChronoDuration, TimeZone, Timelike, Utc};
use cron::Schedule;

use crate::CronError;

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
    /// Returns the Hutool/Calendar field index.
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

    /// Returns the inclusive minimum.
    #[must_use]
    pub const fn min(self) -> i32 {
        match self {
            Self::Second | Self::Minute | Self::Hour | Self::DayOfWeek => 0,
            Self::DayOfMonth | Self::Month => 1,
            Self::Year => 1970,
        }
    }

    /// Returns the inclusive maximum.
    #[must_use]
    pub const fn max(self) -> i32 {
        match self {
            Self::Second | Self::Minute => 59,
            Self::Hour => 23,
            Self::DayOfMonth => 32,  // Hutool: 32 == last day ("L")
            Self::Month => 12,
            Self::DayOfWeek => 7,
            Self::Year => 2099,
        }
    }

    /// Validates and returns a field value.
    pub fn check_value(self, value: i32) -> Result<i32, CronError> {
        if (self.min()..=self.max()).contains(&value) {
            Ok(value)
        } else {
            Err(CronError::InvalidPartValue { part: self, value })
        }
    }

    /// Resolves a field by its zero-based expression index.
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

use super::{apply_negative, checked_schedule_value, convert_hutool_dow_field, convert_hutool_dow_token, end_of_year, expand_field, expand_range, field_needs_expand};
use super::{fields, hutool_dow_to_quartz, is_last_day_of_month, next_after_filtered, normalize_expanded, pad_fields, parse_alias, schedule_max};
use super::{split_numeric_range};
