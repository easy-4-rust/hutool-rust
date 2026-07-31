#![allow(clippy::missing_panics_doc)]
//! 对齐: `cn.hutool.cron.pattern.matcher.PatternMatcher`
//! 来源: hutool-cron/src/main/java/cn/hutool/cron/pattern/matcher/PatternMatcher.java
//! 中文说明: 由 `PatternParser` 组装的七字段匹配器。

use super::part::Part;
use super::part_matcher::PartMatcher;

/// 对齐: `cn.hutool.cron.pattern.matcher.PatternMatcher`
/// 中文说明: 由 `PatternParser` 组装的七字段匹配器。
///
/// Seven-field matcher assembled by `PatternParser`.
#[derive(Debug)]
pub struct PatternMatcher {
    fields: [Box<dyn PartMatcher>; 7],
}

impl PatternMatcher {
    /// 中文说明: 从全部七个字段创建匹配器。
    /// 对齐 Java 方法: `new`
    #[must_use]
    pub fn new(fields: [Box<dyn PartMatcher>; 7]) -> Self {
        Self { fields }
    }

    /// 中文说明: 返回指定字段的匹配器。
    /// 对齐 Java 方法: `get`
    #[must_use]
    pub fn get(&self, part: Part) -> &dyn PartMatcher {
        self.fields[part.calendar_field()].as_ref()
    }

    /// 中文说明: 匹配 `[秒, 分, 时, 日, 月, 周, 年]` 七个字段。
    /// 对齐 Java 方法: `match`
    #[must_use]
    pub fn matches(&self, fields: [i32; 7]) -> bool {
        self.fields
            .iter()
            .zip(fields)
            .all(|(matcher, value)| matcher.matches(value))
    }

    /// 中文说明: 匹配 Java/Hutool 星期编号（0 和 7 均表示周日）。
    /// 对齐 Java 方法: `match`
    #[must_use]
    pub fn matches_week(&self, weekday: i32) -> bool {
        let matcher = self.get(Part::DayOfWeek);
        matcher.matches(weekday) || (weekday == 0 && matcher.matches(7))
    }
}
