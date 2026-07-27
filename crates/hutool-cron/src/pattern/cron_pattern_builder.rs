#![allow(clippy::missing_panics_doc)]
//! 对齐: `cn.hutool.cron.pattern.CronPatternBuilder`
//! 来源: hutool-cron/src/main/java/cn/hutool/cron/pattern/CronPatternBuilder.java
//! 中文说明: 增量构建 Hutool 风格 cron 表达式的构建器。

use std::{fmt, str::FromStr};

use chrono::{DateTime, Datelike, Duration as ChronoDuration, TimeZone, Timelike, Utc};
use cron::Schedule;

use crate::CronError;

use super::cron_pattern::CronPattern;
use super::part::Part;

/// 对齐: `cn.hutool.cron.pattern.CronPatternBuilder`
/// 中文说明: 增量构建 Hutool 风格 cron 表达式的构建器，
/// 未设置的秒/年字段在构建时会被忽略（`NullMode.IGNORE`）。
///
/// Incrementally builds a Hutool-style cron expression.
#[derive(Debug, Clone, Default)]
pub struct CronPatternBuilder {
    parts: [Option<String>; 7],
}

impl CronPatternBuilder {
    /// 中文说明: 创建空构建器（分至周字段构建时默认为 `*`）。
    /// 对齐 Java 方法: `new`
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// 中文说明: 匹配 Hutool `CronPatternBuilder.of()` 的别名。
    /// 对齐 Java 方法: `of`
    #[must_use]
    pub fn of() -> Self {
        Self::new()
    }

    /// 中文说明: 设置逗号分隔的值集合。
    /// 对齐 Java 方法: `setValues`
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

    /// 中文说明: 设置值范围。当 `begin > end` 时保留 Hutool 回绕表示法。
    /// 对齐 Java 方法: `setRange`
    pub fn set_range(&mut self, part: Part, begin: i32, end: i32) -> Result<&mut Self, CronError> {
        part.check_value(begin)?;
        part.check_value(end)?;
        self.parts[part.calendar_field()] = Some(format!("{begin}-{end}"));
        Ok(self)
    }

    /// 中文说明: 设置原始字段值（经解析引擎验证）。
    /// 对齐 Java 方法: `set`
    pub fn set(&mut self, part: Part, value: impl Into<String>) -> Result<&mut Self, CronError> {
        let value = value.into();
        let mut candidate = self.clone();
        candidate.parts[part.calendar_field()] = Some(value);
        CronPattern::parse(candidate.build())?;
        *self = candidate;
        Ok(self)
    }

    /// 中文说明: 构建表达式，未设置的秒/年字段如 Hutool 一样被忽略。
    /// 对齐 Java 方法: `build`
    #[must_use]
    pub fn build(&self) -> String {
        let mut parts = self.parts.clone();
        // From minute through day-of-week, unset fields default to `*`.
        for index in Part::Minute.calendar_field()..Part::Year.calendar_field() {
            if parts[index]
                .as_ref()
                .is_none_or(|value| value.trim().is_empty())
            {
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

use super::split_numeric_range;
use super::{
    apply_negative, checked_schedule_value, convert_hutool_dow_field, convert_hutool_dow_token,
    end_of_year, expand_field, expand_range, field_needs_expand,
};
use super::{
    fields, hutool_dow_to_quartz, is_last_day_of_month, next_after_filtered, normalize_expanded,
    pad_fields, parse_alias, schedule_max,
};
