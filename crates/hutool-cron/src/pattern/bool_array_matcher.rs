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
