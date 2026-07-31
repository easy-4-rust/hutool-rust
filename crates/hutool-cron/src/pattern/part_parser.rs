#![allow(clippy::missing_panics_doc)]
//! 对齐: `cn.hutool.cron.pattern.PartParser`
//! 来源: hutool-cron/src/main/java/cn/hutool/cron/pattern/PartParser.java
//! 中文说明: 将单个 cron 字段解析为匹配器。

use crate::CronError;

use super::always_true_matcher::AlwaysTrueMatcher;
use super::bool_array_matcher::BoolArrayMatcher;
use super::day_of_month_matcher::DayOfMonthMatcher;
use super::part::Part;
use super::part_matcher::PartMatcher;
use super::year_value_matcher::YearValueMatcher;

/// 对齐: `cn.hutool.cron.pattern.PartParser`
/// 中文说明: 将单个 cron 字段解析为匹配器。
///
/// Parses a single cron part into a matcher.
#[derive(Debug, Clone, Copy)]
pub struct PartParser {
    part: Part,
}

impl PartParser {
    /// 中文说明: 为指定字段创建解析器。
    /// 对齐 Java 方法: `new`
    #[must_use]
    pub const fn new(part: Part) -> Self {
        Self { part }
    }

    /// 中文说明: 解析通配符、列表、范围、步长、`L`、负数和回绕范围。
    /// 对齐 Java 方法: `parse`
    pub fn parse(&self, value: &str) -> Result<Box<dyn PartMatcher>, CronError> {
        if matches!(value, "*" | "?") {
            return Ok(Box::new(AlwaysTrueMatcher));
        }
        let mut values = Vec::new();
        for item in value.split(',') {
            let (base, step) = item.split_once('/').map_or((item, 1), |(base, step)| {
                (base, step.parse::<i32>().unwrap_or(0))
            });
            if step <= 0 {
                return Err(CronError::InvalidPattern(value.to_owned()));
            }
            let range_max = if self.part == Part::DayOfMonth {
                31
            } else {
                self.part.max()
            };
            let collected = if base == "*" {
                expand_range(self.part, self.part.min(), range_max, step)?
            } else if let Some((begin, end)) = base
                .split_once('-')
                .filter(|(b, e)| !b.is_empty() && !e.is_empty())
            {
                let begin = apply_negative(self.part, parse_alias(self.part, begin)?)?;
                let end = apply_negative(self.part, parse_alias(self.part, end)?)?;
                expand_range(self.part, begin, end, step)?
            } else {
                let begin = apply_negative(self.part, parse_alias(self.part, base)?)?;
                if step > 1 {
                    expand_range(self.part, begin, range_max, step)?
                } else if self.part == Part::DayOfMonth && begin == 32 {
                    vec![32]
                } else {
                    vec![checked_schedule_value(self.part, begin)?]
                }
            };
            values.extend(collected);
        }
        if self.part == Part::Year {
            Ok(Box::new(YearValueMatcher::from_values(values).expect(
                "year values were validated while parsing the cron field",
            )))
        } else if self.part == Part::DayOfMonth {
            Ok(Box::new(DayOfMonthMatcher::from_values(values).expect(
                "day-of-month values were validated while parsing the cron field",
            )))
        } else {
            Ok(Box::new(BoolArrayMatcher::from_values(values).expect(
                "a successfully parsed finite cron field contains a value",
            )))
        }
    }
}

use super::{apply_negative, checked_schedule_value, expand_range, parse_alias};
