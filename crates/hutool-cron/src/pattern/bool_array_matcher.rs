#![allow(clippy::missing_panics_doc)]
//! Hutool-aligned cron patterns, builders, parsers, and matchers.


use std::{fmt, str::FromStr};

use chrono::{DateTime, Datelike, Duration as ChronoDuration, TimeZone, Timelike, Utc};
use cron::Schedule;

use crate::CronError;

use super::part_matcher::PartMatcher;

/// Sorted finite-value matcher used for most cron fields.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoolArrayMatcher {
    values: Vec<i32>,
}

impl BoolArrayMatcher {
    /// Creates a matcher from a non-empty value collection.
    pub fn new(values: impl IntoIterator<Item = i32>) -> Result<Self, CronError> {
        Self::from_values(values.into_iter().collect())
    }

    pub(crate) fn from_values(mut values: Vec<i32>) -> Result<Self, CronError> {
        values.sort_unstable();
        values.dedup();
        if values.is_empty() {
            return Err(CronError::EmptyMatcher);
        }
        Ok(Self { values })
    }

    /// Returns the minimum represented value.
    #[must_use]
    pub fn min_value(&self) -> i32 {
        self.values[0]
    }

    /// Returns the maximum represented value.
    #[must_use]
    pub fn max_value(&self) -> i32 {
        self.values[self.values.len() - 1]
    }
}

impl PartMatcher for BoolArrayMatcher {
    fn matches(&self, value: i32) -> bool {
        self.values.binary_search(&value).is_ok()
    }

    fn next_after(&self, value: i32) -> i32 {
        self.values
            .iter()
            .copied()
            .find(|candidate| *candidate >= value)
            .unwrap_or_else(|| self.min_value())
    }
}

impl fmt::Display for BoolArrayMatcher {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let values = self.values.iter().map(i32::to_string).collect::<Vec<_>>();
        formatter.write_str(&values.join(","))
    }
}

use super::{apply_negative, checked_schedule_value, convert_hutool_dow_field, convert_hutool_dow_token, end_of_year, expand_field, expand_range, field_needs_expand};
use super::{fields, hutool_dow_to_quartz, is_last_day_of_month, next_after_filtered, normalize_expanded, pad_fields, parse_alias, schedule_max};
use super::{split_numeric_range};
