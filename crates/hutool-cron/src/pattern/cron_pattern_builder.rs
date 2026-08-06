#![allow(clippy::missing_panics_doc)]
//! 对齐: `cn.hutool.cron.pattern.CronPatternBuilder`
//! 来源: hutool-cron/src/main/java/cn/hutool/cron/pattern/CronPatternBuilder.java
//! 中文说明: 增量构建 Hutool 风格 cron 表达式的构建器。

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_match_all_matches_java_test() {
        // Java CronPatternBuilderTest.buildMatchAllTest()
        let build = CronPatternBuilder::of().build();
        assert_eq!(build, "* * * * *");

        let build = CronPatternBuilder::of()
            .set(Part::Second, "*")
            .unwrap()
            .build();
        assert_eq!(build, "* * * * * *");

        let build = CronPatternBuilder::of()
            .set(Part::Second, "*")
            .unwrap()
            .set(Part::Year, "*")
            .unwrap()
            .build();
        assert_eq!(build, "* * * * * * *");
    }

    #[test]
    fn build_range_matches_java_test() {
        // Java CronPatternBuilderTest.buildRangeTest()
        let build = CronPatternBuilder::of()
            .set(Part::Second, "*")
            .unwrap()
            .set_range(Part::Hour, 2, 9)
            .unwrap()
            .build();
        assert_eq!(build, "* * 2-9 * * *");
    }

    #[test]
    fn build_range_error_matches_java_test() {
        // Java CronPatternBuilderTest.buildRangeErrorTest()：55 超出小时范围报错
        let mut builder = CronPatternBuilder::of();
        builder.set(Part::Second, "*").unwrap();
        assert!(builder.set_range(Part::Hour, 2, 55).is_err());
    }

    #[test]
    fn set_values_and_empty_errors() {
        let mut builder = CronPatternBuilder::of();
        // 空值集合报错
        assert!(builder.set_values(Part::Hour, &[]).is_err());
        // 值越界报错
        assert!(builder.set_values(Part::Hour, &[25]).is_err());
        // 合法值集合
        builder.set_values(Part::Hour, &[1, 2, 5]).unwrap();
        // Minute 未设置时默认 "*"，Hour 被设置为值集合
        assert_eq!(builder.build(), "* 1,2,5 * * *");
    }
}
