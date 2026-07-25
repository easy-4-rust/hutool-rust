#![allow(clippy::missing_panics_doc)]
//! Hutool-aligned cron patterns, builders, parsers, and matchers.


use std::{fmt, str::FromStr};

use chrono::{DateTime, Datelike, Duration as ChronoDuration, TimeZone, Timelike, Utc};
use cron::Schedule;

use crate::CronError;

use super::part::Part;
use super::part_matcher::PartMatcher;
use super::pattern_parser::PatternParser;

/// Seven-field matcher assembled by `PatternParser`.
#[derive(Debug)]
pub struct PatternMatcher {
    fields: [Box<dyn PartMatcher>; 7],
}

impl PatternMatcher {
    /// Creates a matcher from all seven fields.
    #[must_use]
    pub fn new(fields: [Box<dyn PartMatcher>; 7]) -> Self {
        Self { fields }
    }

    /// Returns a field matcher.
    #[must_use]
    pub fn get(&self, part: Part) -> &dyn PartMatcher {
        self.fields[part.calendar_field()].as_ref()
    }

    /// Matches `[second, minute, hour, day, month, weekday, year]`.
    #[must_use]
    pub fn matches(&self, fields: [i32; 7]) -> bool {
        self.fields
            .iter()
            .zip(fields)
            .all(|(matcher, value)| matcher.matches(value))
    }

    /// Matches Java/Hutool weekday numbering, treating 0 and 7 as Sunday.
    #[must_use]
    pub fn matches_week(&self, weekday: i32) -> bool {
        let matcher = self.get(Part::DayOfWeek);
        matcher.matches(weekday) || (weekday == 0 && matcher.matches(7))
    }
}

use super::{apply_negative, checked_schedule_value, convert_hutool_dow_field, convert_hutool_dow_token, end_of_year, expand_field, expand_range, field_needs_expand};
use super::{fields, hutool_dow_to_quartz, is_last_day_of_month, next_after_filtered, normalize_expanded, pad_fields, parse_alias, schedule_max};
use super::{split_numeric_range};
