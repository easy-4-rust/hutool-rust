#![allow(clippy::missing_panics_doc)]
//! 对齐: `cn.hutool.cron.pattern.matcher.YearValueMatcher`
//! 来源: hutool-cron/src/main/java/cn/hutool/cron/pattern/matcher/YearValueMatcher.java
//! 中文说明: 具有 Hutool 下限行为的年份匹配器。

use crate::CronError;

use super::bool_array_matcher::BoolArrayMatcher;
use super::part::Part;
use super::part_matcher::PartMatcher;

/// 对齐: `cn.hutool.cron.pattern.matcher.YearValueMatcher`
/// 中文说明: 具有 Hutool 下限行为的年份匹配器。
///
/// Year matcher with Hutool's lower-bound behavior.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YearValueMatcher(BoolArrayMatcher);

impl YearValueMatcher {
    /// 中文说明: 创建年份匹配器。
    /// 对齐 Java 方法: `new`
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn year_matcher_match_and_next() {
        let matcher = YearValueMatcher::new([2026, 2030]).unwrap();
        assert!(matcher.matches(2026));
        assert!(matcher.matches(2030));
        assert!(!matcher.matches(2027));
        assert_eq!(matcher.next_after(2027), 2030);
        assert_eq!(matcher.next_after(2031), 2026);
    }

    #[test]
    fn year_matcher_validates_range() {
        // 年份越界（超出 Part::Year 范围）报错
        assert!(YearValueMatcher::new([0]).is_err());
        assert!(YearValueMatcher::new([i32::MAX]).is_err());
    }
}
