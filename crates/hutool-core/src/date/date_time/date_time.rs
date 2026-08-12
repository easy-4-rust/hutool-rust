//! 对齐: `cn.hutool.core.date.DateTime`
//!
//! # Timezone note
//! 默认按 UTC+08:00（Asia/Shanghai 无夏令时偏移）解释/格式化墙钟时间，
//! 与 Hutool `DateUtilTest` 的 `TZ=Asia/Shanghai` 约定一致。

#![allow(dead_code)]

use crate::date::week::Week;

/// 对齐 Java: `cn.hutool.core.date.DateTime`
#[derive(Debug, Clone, Copy)]
pub struct DateTime {
    /// epoch 毫秒
    pub(crate) millis: i64,
    /// 一周起始（默认周一，便于 beginOfWeek 与常见中国习惯一致；可 set）
    pub(crate) first_day_of_week: Week,
    /// 可变模式（Hutool mutable）
    pub(crate) mutable: bool,
}
