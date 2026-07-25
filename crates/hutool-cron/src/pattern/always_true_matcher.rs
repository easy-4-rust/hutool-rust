#![allow(clippy::missing_panics_doc)]
//! Hutool-aligned cron patterns, builders, parsers, and matchers.


use std::{fmt, str::FromStr};

use chrono::{DateTime, Datelike, Duration as ChronoDuration, TimeZone, Timelike, Utc};
use cron::Schedule;

use crate::CronError;

use super::part_matcher::PartMatcher;

/// Matcher that accepts every value.
#[derive(Debug, Clone, Copy, Default)]
pub struct AlwaysTrueMatcher;

impl PartMatcher for AlwaysTrueMatcher {
    fn matches(&self, _value: i32) -> bool {
        true
    }

    fn next_after(&self, value: i32) -> i32 {
        value
    }
}

impl fmt::Display for AlwaysTrueMatcher {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("*")
    }
}

use super::{apply_negative, checked_schedule_value, convert_hutool_dow_field, convert_hutool_dow_token, end_of_year, expand_field, expand_range, field_needs_expand};
use super::{fields, hutool_dow_to_quartz, is_last_day_of_month, next_after_filtered, normalize_expanded, pad_fields, parse_alias, schedule_max};
use super::{split_numeric_range};
