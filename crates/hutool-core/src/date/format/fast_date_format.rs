//! 对齐: `cn.hutool.core.date.format.FastDateFormat`

use crate::Result;
use crate::date::date_time::DateTime;
use crate::date::date_util::DateUtil;

/// 对齐 Java: `FastDateFormat`
#[derive(Debug, Clone)]
pub struct FastDateFormat {
    pattern: String,
}

impl FastDateFormat {
    /// 获取实例。
    pub fn get_instance(pattern: &str) -> Self {
        Self {
            pattern: pattern.to_string(),
        }
    }

    /// 格式化。
    pub fn format(&self, date: DateTime) -> String {
        DateUtil::format(date, &self.pattern)
    }

    /// 解析。
    pub fn parse(&self, date_str: &str) -> Result<DateTime> {
        DateUtil::parse_with_format(date_str, &self.pattern)
    }

    /// 兼容 sentinel。
    pub fn sentinel() -> Result<()> {
        Ok(())
    }
}
