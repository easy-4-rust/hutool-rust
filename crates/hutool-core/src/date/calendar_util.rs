//! 对齐: `cn.hutool.core.date.CalendarUtil`

#![allow(dead_code)]

use crate::Result;
use crate::date::date_time::DateTime;
use crate::date::date_util::DateUtil;

/// 对齐 Java: `cn.hutool.core.date.CalendarUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct CalendarUtil;

impl CalendarUtil {
    /// 格式化中文日期。
    pub fn format_chinese_date(date: DateTime, with_time: bool) -> String {
        DateUtil::format_chinese_date(date, false, with_time)
    }

    /// 解析为 DateTime。
    pub fn parse(date_str: &str) -> crate::Result<DateTime> {
        DateUtil::parse(date_str)
    }

    /// 兼容 sentinel。
    pub fn sentinel() -> Result<()> {
        Ok(())
    }
}
