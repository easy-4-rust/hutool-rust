#![allow(clippy::missing_panics_doc)]
//! Hutool-aligned cron patterns, builders, parsers, and matchers.


use std::{fmt, str::FromStr};

use chrono::{DateTime, Datelike, Duration as ChronoDuration, TimeZone, Timelike, Utc};
use cron::Schedule;

use crate::CronError;

use super::cron_pattern::CronPattern;
use super::part::Part;

/// Incrementally builds a Hutool-style cron expression.
///
/// Unset second/year fields are omitted from [`Self::build`], matching Hutool
/// `CronPatternBuilder` (`NullMode.IGNORE`).
#[derive(Debug, Clone, Default)]
pub struct CronPatternBuilder {
    parts: [Option<String>; 7],
}

impl CronPatternBuilder {
    /// Creates an empty builder (minute–week default to `*` on build).
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Alias for [`Self::new`], matching Hutool `CronPatternBuilder.of()`.
    #[must_use]
    pub fn of() -> Self {
        Self::new()
    }

    /// Sets a comma-separated collection of values.
    pub fn set_values(&mut self, part: Part, values: &[i32]) -> Result<&mut Self, CronError> {
        if values.is_empty() {
            return Err(CronError::EmptyPartValues(part));
        }
        let values = values
            .iter()
            .map(|value| part.check_value(*value).map(|value| value.to_string()))
            .collect::<Result<Vec<_>, _>>()?;
        self.parts[part.calendar_field()] = Some(values.join(","));
        Ok(self)
    }

    /// Sets a value range. When `begin > end`, Hutool wrap notation is kept.
    pub fn set_range(&mut self, part: Part, begin: i32, end: i32) -> Result<&mut Self, CronError> {
        part.check_value(begin)?;
        part.check_value(end)?;
        self.parts[part.calendar_field()] = Some(format!("{begin}-{end}"));
        Ok(self)
    }

    /// Sets a raw field after validating it with the parser engine.
    pub fn set(&mut self, part: Part, value: impl Into<String>) -> Result<&mut Self, CronError> {
        let value = value.into();
        let mut candidate = self.clone();
        candidate.parts[part.calendar_field()] = Some(value);
        CronPattern::parse(candidate.build())?;
        *self = candidate;
        Ok(self)
    }

    /// Builds the expression, omitting unset second/year like Hutool.
    #[must_use]
    pub fn build(&self) -> String {
        let mut parts = self.parts.clone();
        // From minute through day-of-week, unset fields default to `*`.
        for index in Part::Minute.calendar_field()..Part::Year.calendar_field() {
            if parts[index].as_ref().is_none_or(|value| value.trim().is_empty()) {
                parts[index] = Some("*".to_owned());
            }
        }
        parts
            .into_iter()
            .flatten()
            .filter(|value| !value.trim().is_empty())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

use super::{apply_negative, checked_schedule_value, convert_hutool_dow_field, convert_hutool_dow_token, end_of_year, expand_field, expand_range, field_needs_expand};
use super::{fields, hutool_dow_to_quartz, is_last_day_of_month, next_after_filtered, normalize_expanded, pad_fields, parse_alias, schedule_max};
use super::{split_numeric_range};
