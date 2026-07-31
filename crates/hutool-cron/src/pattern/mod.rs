#![allow(clippy::missing_panics_doc)]
//! 对齐: `cn.hutool.cron.pattern`
//! 来源: hutool-cron/src/main/java/cn/hutool/cron/pattern/
//! 中文说明: Hutool 定时任务表达式解析模块，包含表达式构建器、
//! 解析器、字段匹配器等核心组件。

use chrono::{DateTime, Datelike, TimeZone, Timelike, Utc};
use cron::Schedule;

use crate::CronError;

mod always_true_matcher;
mod bool_array_matcher;
mod cron_pattern;
mod cron_pattern_builder;
mod cron_pattern_util;
mod day_of_month_matcher;
mod part;
mod part_matcher;
mod part_parser;
mod pattern_matcher;
mod pattern_parser;
mod year_value_matcher;

pub use always_true_matcher::AlwaysTrueMatcher;
pub use bool_array_matcher::BoolArrayMatcher;
pub use cron_pattern::CronPattern;
pub use cron_pattern_builder::CronPatternBuilder;
pub use cron_pattern_util::CronPatternUtil;
pub use day_of_month_matcher::DayOfMonthMatcher;
pub use part::Part;
pub use part_matcher::PartMatcher;
pub use part_parser::PartParser;
pub use pattern_matcher::PatternMatcher;
pub use pattern_parser::PatternParser;
pub use year_value_matcher::YearValueMatcher;

fn next_after_filtered(
    schedule: &Schedule,
    start: DateTime<Utc>,
    dom_last: bool,
) -> Option<DateTime<Utc>> {
    let mut cursor = start;
    // Bound iterations: worst case skips ~3 days per month when expanding L→28..31.
    for _ in 0..50_000 {
        let next = schedule.after(&cursor).next()?;
        if !dom_last || is_last_day_of_month(next) {
            return Some(next);
        }
        cursor = next;
    }
    None
}

fn is_last_day_of_month(instant: DateTime<Utc>) -> bool {
    let leap = instant.year() % 4 == 0 && (instant.year() % 100 != 0 || instant.year() % 400 == 0);
    instant.day() == DayOfMonthMatcher::last_day(instant.month(), leap)
}

fn normalize_expanded(expression: &str, match_second: bool) -> Result<(String, bool), CronError> {
    let mut fields = expression.split_whitespace().collect::<Vec<_>>();
    match fields.len() {
        5 => {
            fields.insert(0, "0");
            fields.push("*");
        }
        6 => fields.push("*"),
        7 => {}
        _ => return Err(CronError::InvalidPattern(expression.to_owned())),
    }
    if !match_second {
        fields[0] = "0";
    }
    let parts = [
        Part::Second,
        Part::Minute,
        Part::Hour,
        Part::DayOfMonth,
        Part::Month,
        Part::DayOfWeek,
        Part::Year,
    ];
    let mut dom_last = false;
    let mut expanded = Vec::with_capacity(7);
    for (part, field) in parts.into_iter().zip(fields.iter().copied()) {
        let (field_expr, last) = expand_field(part, field)?;
        if part == Part::DayOfMonth {
            dom_last = last;
        }
        expanded.push(field_expr);
    }
    Ok((expanded.join(" "), dom_last))
}

fn field_needs_expand(field: &str) -> bool {
    for item in field.split(',') {
        let base = item.split_once('/').map_or(item, |(base, _)| base);
        if base.eq_ignore_ascii_case("l") {
            return true;
        }
        // Lone negative number: `-4`
        if base.starts_with('-') && base.len() > 1 && base[1..].chars().all(|c| c.is_ascii_digit())
        {
            return true;
        }
        // Wrapping numeric range: `22-2`
        if let Some((begin, end)) = split_numeric_range(base) {
            if begin > end {
                return true;
            }
        }
    }
    false
}

fn split_numeric_range(base: &str) -> Option<(i32, i32)> {
    let (begin, end) = base.split_once('-')?;
    if begin.is_empty() || end.is_empty() {
        return None;
    }
    Some((begin.parse().ok()?, end.parse().ok()?))
}

fn convert_hutool_dow_field(field: &str) -> Result<String, CronError> {
    let mut out = Vec::new();
    for item in field.split(',') {
        let (base, step) = item
            .split_once('/')
            .map_or((item, None), |(b, s)| (b, Some(s)));
        let converted = if let Some((begin, end)) = base
            .split_once('-')
            .filter(|(b, e)| !b.is_empty() && !e.is_empty())
        {
            format!(
                "{}-{}",
                convert_hutool_dow_token(begin)?,
                convert_hutool_dow_token(end)?
            )
        } else {
            convert_hutool_dow_token(base)?
        };
        if let Some(step) = step {
            out.push(format!("{converted}/{step}"));
        } else {
            out.push(converted);
        }
    }
    Ok(out.join(","))
}

fn convert_hutool_dow_token(token: &str) -> Result<String, CronError> {
    if token.chars().all(|c| c.is_ascii_digit()) {
        let value: i32 = token
            .parse()
            .map_err(|_| CronError::InvalidPattern(token.to_owned()))?;
        return Ok(hutool_dow_to_quartz(value)?.to_string());
    }
    // Keep Sun/Mon/... aliases for the schedule engine.
    Ok(token.to_owned())
}

fn expand_field(part: Part, field: &str) -> Result<(String, bool), CronError> {
    if matches!(field, "*" | "?") {
        return Ok((field.to_owned(), false));
    }
    // Preserve star-step forms (`*/5`) for the schedule engine.
    if let Some(rest) = field.strip_prefix("*/") {
        let step: i32 = rest
            .parse()
            .map_err(|_| CronError::InvalidPattern(field.to_owned()))?;
        if step <= 0 {
            return Err(CronError::InvalidPattern(field.to_owned()));
        }
        return Ok((format!("*/{step}"), false));
    }
    // Hutool DOW is 0-6/7 (Sun=0/7); the `cron` crate uses Quartz 1-7 (Sun=1).
    if part == Part::DayOfWeek && !field_needs_expand(field) {
        return Ok((convert_hutool_dow_field(field)?, false));
    }
    if !field_needs_expand(field) {
        return Ok((field.to_owned(), false));
    }

    let mut values = Vec::new();
    let mut has_last = false;
    for item in field.split(',') {
        let (base, step) = item.split_once('/').map_or((item, 1_i32), |(base, step)| {
            (base, step.parse::<i32>().unwrap_or(0))
        });
        if step <= 0 {
            return Err(CronError::InvalidPattern(field.to_owned()));
        }
        let collected = if base == "*" {
            expand_range(part, part.min(), schedule_max(part), step)?
        } else if let Some((begin, end)) = base
            .split_once('-')
            .filter(|(b, e)| !b.is_empty() && !e.is_empty())
        {
            let begin = apply_negative(part, parse_alias(part, begin)?)?;
            let end = apply_negative(part, parse_alias(part, end)?)?;
            expand_range(part, begin, end, step)?
        } else {
            let value = apply_negative(part, parse_alias(part, base)?)?;
            if part == Part::DayOfMonth && value == 32 {
                has_last = true;
                Vec::new()
            } else if step > 1 {
                expand_range(part, value, schedule_max(part), step)?
            } else {
                vec![checked_schedule_value(part, value)?]
            }
        };
        for value in collected {
            if part == Part::DayOfMonth && value == 32 {
                has_last = true;
            } else {
                values.push(value);
            }
        }
    }
    if has_last && part == Part::DayOfMonth {
        values.extend(28..=31);
    }
    values.sort_unstable();
    values.dedup();
    if values.is_empty() {
        return Err(CronError::InvalidPattern(field.to_owned()));
    }
    if part == Part::DayOfWeek {
        values = values
            .into_iter()
            .map(hutool_dow_to_quartz)
            .collect::<Result<Vec<_>, _>>()?;
        values.sort_unstable();
        values.dedup();
    }
    Ok((
        values
            .iter()
            .map(i32::to_string)
            .collect::<Vec<_>>()
            .join(","),
        has_last,
    ))
}

fn hutool_dow_to_quartz(value: i32) -> Result<i32, CronError> {
    match value {
        0 | 7 => Ok(1),
        1..=6 => Ok(value + 1),
        _ => Err(CronError::InvalidPartValue {
            part: Part::DayOfWeek,
            value,
        }),
    }
}

fn schedule_max(part: Part) -> i32 {
    if part == Part::DayOfMonth {
        31
    } else {
        part.max()
    }
}

fn checked_schedule_value(part: Part, value: i32) -> Result<i32, CronError> {
    if part == Part::DayOfMonth {
        if value == 32 {
            return Ok(32);
        }
        if !(1..=31).contains(&value) {
            return Err(CronError::InvalidPartValue { part, value });
        }
        return Ok(value);
    }
    part.check_value(value)
}

fn expand_range(part: Part, begin: i32, end: i32, step: i32) -> Result<Vec<i32>, CronError> {
    let step = usize::try_from(step).expect("positive step fits usize");
    let max = if part == Part::DayOfMonth {
        31
    } else {
        part.max()
    };
    let min = part.min();
    // For DOM, allow 32 only as standalone L, not in numeric ranges beyond 31.
    let begin = if part == Part::DayOfMonth && begin == 32 {
        max
    } else {
        begin
    };
    let end = if part == Part::DayOfMonth && end == 32 {
        max
    } else {
        end
    };
    if part == Part::DayOfMonth {
        if !(1..=31).contains(&begin) || !(1..=31).contains(&end) {
            return Err(CronError::InvalidPartValue {
                part,
                value: begin.max(end),
            });
        }
    } else {
        part.check_value(begin)?;
        part.check_value(end)?;
    }
    let mut values = Vec::new();
    if begin <= end {
        values.extend((begin..=end).step_by(step));
    } else {
        // Hutool wrap: 22-2 → 22..=max then min..=2
        values.extend((begin..=max).step_by(step));
        values.extend((min..=end).step_by(step));
    }
    Ok(values)
}

fn apply_negative(part: Part, value: i32) -> Result<i32, CronError> {
    if value >= 0 {
        return Ok(value);
    }
    // Hutool: `i += part.getMax()` — hour `-4` → 19; DOM uses max 32.
    Ok(value + part.max())
}

fn end_of_year(start: DateTime<Utc>) -> DateTime<Utc> {
    Utc.with_ymd_and_hms(start.year(), 12, 31, 23, 59, 59)
        .single()
        .unwrap_or(start)
}

fn parse_alias(part: Part, value: &str) -> Result<i32, CronError> {
    let lowercase = value.to_ascii_lowercase();
    // Hutool: `L` means the field maximum (day-of-month sentinel 32, Saturday=6).
    if lowercase == "l" {
        return Ok(match part {
            Part::DayOfMonth => 32,
            Part::DayOfWeek => 6,
            _ => part.max(),
        });
    }
    let alias = match part {
        Part::Month => [
            "jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec",
        ]
        .iter()
        .position(|candidate| *candidate == lowercase)
        .and_then(|index| i32::try_from(index).ok())
        .map(|index| index + 1),
        Part::DayOfWeek => ["sun", "mon", "tue", "wed", "thu", "fri", "sat"]
            .iter()
            .position(|candidate| *candidate == lowercase)
            .and_then(|index| i32::try_from(index).ok()),
        _ => None,
    };
    alias
        .or_else(|| value.parse().ok())
        .ok_or_else(|| CronError::InvalidPattern(value.to_owned()))
}

fn pad_fields(expression: &str, match_second: bool) -> Result<[String; 7], CronError> {
    let mut fields = expression
        .split_whitespace()
        .map(str::to_owned)
        .collect::<Vec<_>>();
    match fields.len() {
        5 => {
            fields.insert(0, "0".to_owned());
            fields.push("*".to_owned());
        }
        6 => fields.push("*".to_owned()),
        7 => {}
        _ => return Err(CronError::InvalidPattern(expression.to_owned())),
    }
    if !match_second {
        fields[0] = "0".to_owned();
    }
    Ok([
        fields[0].clone(),
        fields[1].clone(),
        fields[2].clone(),
        fields[3].clone(),
        fields[4].clone(),
        fields[5].clone(),
        fields[6].clone(),
    ])
}

/// 中文说明: 提取时刻的七字段数组 `[秒, 分, 时, 日, 月, 周, 年]`，
/// 与 Hutool `CronPatternUtil.matchedDates` 的字段提取逻辑对应。
///
/// Builds a seven-field `[sec, min, hour, day, month, weekday, year]` array
/// from a timestamp.
pub fn fields<Tz: TimeZone>(value: &DateTime<Tz>, match_second: bool) -> [i32; 7] {
    [
        if match_second {
            i32::try_from(value.second()).unwrap_or_default()
        } else {
            0
        },
        i32::try_from(value.minute()).unwrap_or_default(),
        i32::try_from(value.hour()).unwrap_or_default(),
        i32::try_from(value.day()).unwrap_or_default(),
        i32::try_from(value.month()).unwrap_or_default(),
        i32::try_from(value.weekday().num_days_from_sunday()).unwrap_or_default(),
        value.year(),
    ]
}
