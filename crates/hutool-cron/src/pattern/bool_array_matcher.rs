#![allow(clippy::missing_panics_doc)]
//! 对齐: `cn.hutool.cron.pattern.matcher.BoolArrayMatcher`
//! 来源: hutool-cron/src/main/java/cn/hutool/cron/pattern/matcher/BoolArrayMatcher.java
//! 中文说明: 基于有序有限值集合的字段匹配器，适用于大多数 cron 字段。

use std::fmt;

use crate::CronError;

use super::part_matcher::PartMatcher;

/// 对齐: `cn.hutool.cron.pattern.matcher.BoolArrayMatcher`
/// 中文说明: 基于有序有限值集合的字段匹配器，适用于大多数 cron 字段。
///
/// Sorted finite-value matcher used for most cron fields.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoolArrayMatcher {
    values: Vec<i32>,
}

impl BoolArrayMatcher {
    /// 中文说明: 从非空值集合创建匹配器。
    /// 对齐 Java 方法: `new`
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

    /// 中文说明: 返回最小匹配值。
    /// 对齐 Java 方法: `getMinValue`
    #[must_use]
    pub fn min_value(&self) -> i32 {
        self.values[0]
    }

    /// 中文说明: 返回最大匹配值。
    /// 对齐 Java 方法: `getMaxValue`
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bool_array_matcher_match_and_bounds() {
        let matcher = BoolArrayMatcher::new([2, 5, 9]).unwrap();
        assert_eq!(matcher.min_value(), 2);
        assert_eq!(matcher.max_value(), 9);
        assert!(matcher.matches(2));
        assert!(matcher.matches(5));
        assert!(matcher.matches(9));
        assert!(!matcher.matches(3));
        assert!(!matcher.matches(10));
        assert!(!matcher.matches(1));
    }

    #[test]
    fn bool_array_matcher_next_after_matches_java() {
        let matcher = BoolArrayMatcher::new([2, 5, 9]).unwrap();
        // value == maxValue → value
        assert_eq!(matcher.next_after(9), 9);
        // min < value < max：找第一个 >= value 的匹配值
        assert_eq!(matcher.next_after(3), 5);
        assert_eq!(matcher.next_after(5), 5);
        // value <= min：返回最小值
        assert_eq!(matcher.next_after(1), 2);
        assert_eq!(matcher.next_after(2), 2);
        // value > max：回绕到最小值
        assert_eq!(matcher.next_after(10), 2);
    }

    #[test]
    fn bool_array_matcher_empty_rejected_and_deduped() {
        assert!(BoolArrayMatcher::new([] as [i32; 0]).is_err());
        let matcher = BoolArrayMatcher::new([3, 1, 3, 2]).unwrap();
        assert!(matcher.matches(1));
        assert!(matcher.matches(2));
        assert!(matcher.matches(3));
        assert_eq!(matcher.to_string(), "1,2,3");
    }
}
