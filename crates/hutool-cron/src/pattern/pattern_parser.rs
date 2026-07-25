#![allow(clippy::missing_panics_doc)]
//! 对齐: `cn.hutool.cron.pattern.PatternParser`
//! 来源: hutool-cron/src/main/java/cn/hutool/cron/pattern/PatternParser.java
//! 中文说明: 将完整表达式解析为字段匹配器集合。


use std::{fmt, str::FromStr};

use chrono::{DateTime, Datelike, Duration as ChronoDuration, TimeZone, Timelike, Utc};
use cron::Schedule;

use crate::CronError;

use super::part::Part;
use super::part_parser::PartParser;
use super::pattern_matcher::PatternMatcher;

/// 对齐: `cn.hutool.cron.pattern.PatternParser`
/// 中文说明: 将完整表达式解析为字段匹配器集合。
///
/// Parses full expressions into field matchers.
pub struct PatternParser;

impl PatternParser {
    /// 中文说明: 解析每个 `|` 分隔的表达式。
    /// 对齐 Java 方法: `parse`
    pub fn parse(expression: &str) -> Result<Vec<PatternMatcher>, CronError> {
        expression
            .split('|')
            .map(str::trim)
            .map(|alternative| {
                let fields = pad_fields(alternative, true)?;
                let fields = [
                    PartParser::new(Part::Second).parse(&fields[0])?,
                    PartParser::new(Part::Minute).parse(&fields[1])?,
                    PartParser::new(Part::Hour).parse(&fields[2])?,
                    PartParser::new(Part::DayOfMonth).parse(&fields[3])?,
                    PartParser::new(Part::Month).parse(&fields[4])?,
                    PartParser::new(Part::DayOfWeek).parse(&fields[5])?,
                    PartParser::new(Part::Year).parse(&fields[6])?,
                ];
                Ok(PatternMatcher::new(fields))
            })
            .collect()
    }
}

use super::{apply_negative, checked_schedule_value, convert_hutool_dow_field, convert_hutool_dow_token, end_of_year, expand_field, expand_range, field_needs_expand};
use super::{fields, hutool_dow_to_quartz, is_last_day_of_month, next_after_filtered, normalize_expanded, pad_fields, parse_alias, schedule_max};
use super::{split_numeric_range};
