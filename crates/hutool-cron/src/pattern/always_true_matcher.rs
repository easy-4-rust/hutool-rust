#![allow(clippy::missing_panics_doc)]
//! 对齐: `cn.hutool.cron.pattern.matcher.AlwaysTrueMatcher`
//! 来源: hutool-cron/src/main/java/cn/hutool/cron/pattern/matcher/AlwaysTrueMatcher.java
//! 中文说明: 匹配所有值的通配字段匹配器。

use std::fmt;

use super::part_matcher::PartMatcher;

/// 对齐: `cn.hutool.cron.pattern.matcher.AlwaysTrueMatcher`
/// 中文说明: 匹配所有值的通配字段匹配器。
///
/// Matcher that accepts every value.
#[derive(Debug, Clone, Copy, Default)]
pub struct AlwaysTrueMatcher;

impl PartMatcher for AlwaysTrueMatcher {
    fn matches(&self, _value: i32) -> bool {
        true
    }

    fn next_after(&self, value: i32) -> i32 {
        value
    }
}

impl fmt::Display for AlwaysTrueMatcher {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 对齐 Java toString: "[Matcher]: always true."
        formatter.write_str("[Matcher]: always true.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn always_true_matcher_matches_everything() {
        let matcher = AlwaysTrueMatcher;
        assert!(matcher.matches(0));
        assert!(matcher.matches(i32::MAX));
        assert!(matcher.matches(-1));
        // nextAfter 返回原值
        assert_eq!(matcher.next_after(7), 7);
        assert_eq!(matcher.next_after(-3), -3);
        // toString 对齐 Java
        assert_eq!(matcher.to_string(), "[Matcher]: always true.");
    }
}
