#![allow(clippy::missing_panics_doc)]
//! Hutool-aligned cron patterns, builders, parsers, and matchers.


use std::{fmt, str::FromStr};

use chrono::{DateTime, Datelike, Duration as ChronoDuration, TimeZone, Timelike, Utc};
use cron::Schedule;

use crate::CronError;

use super::bool_array_matcher::BoolArrayMatcher;
use super::part::Part;
use super::part_matcher::PartMatcher;

/// Year matcher with Hutool's lower-bound behavior.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YearValueMatcher(BoolArrayMatcher);

impl YearValueMatcher {
    /// Creates a year matcher.
    pub fn new(values: impl IntoIterator<Item = i32>) -> Result<Self, CronError> {
        Self::from_values(values.into_iter().collect())
    }

    pub(crate) fn from_values(values: Vec<i32>) -> Result<Self, CronError> {
        let mut checked = Vec::new();
        for value in values {
            checked.push(Part::Year.check_value(value)?);
        }
        Ok(Self(BoolArrayMatcher::new(checked)?))
    }
}

impl PartMatcher for YearValueMatcher {
    fn matches(&self, value: i32) -> bool {
        self.0.matches(value)
    }

    fn next_after(&self, value: i32) -> i32 {
        self.0.next_after(value)
    }
}

use super::{apply_negative, checked_schedule_value, convert_hutool_dow_field, convert_hutool_dow_token, end_of_year, expand_field, expand_range, field_needs_expand};
use super::{fields, hutool_dow_to_quartz, is_last_day_of_month, next_after_filtered, normalize_expanded, pad_fields, parse_alias, schedule_max};
use super::{split_numeric_range};
