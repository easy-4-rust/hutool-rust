#![allow(clippy::missing_panics_doc)]
//! 对齐: `cn.hutool.cron.pattern.matcher.PartMatcher`
//! 来源: hutool-cron/src/main/java/cn/hutool/cron/pattern/matcher/PartMatcher.java
//! 中文说明: Cron 表达式单字段匹配器的公共行为接口。

use std::{fmt, str::FromStr};

use chrono::{DateTime, Datelike, Duration as ChronoDuration, TimeZone, Timelike, Utc};
use cron::Schedule;

use crate::CronError;

/// 对齐: `cn.hutool.cron.pattern.matcher.PartMatcher`
/// 中文说明: Cron 表达式单字段匹配器的公共行为接口。
///
/// Common behavior of one cron field matcher.
pub trait PartMatcher: fmt::Debug + Send + Sync {
    /// 中文说明: 返回指定值是否匹配。
    /// 对齐 Java 方法: `match`
    fn matches(&self, value: i32) -> bool;
    /// 中文说明: 返回大于等于指定值的首个匹配值，溢出时回绕到最小值。
    /// 对齐 Java 方法: `nextAfter`
    fn next_after(&self, value: i32) -> i32;
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
