//! 对齐: `cn.hutool.core.date.ZoneUtil`

#![allow(dead_code)]

use chrono::FixedOffset;

use crate::Result;
use crate::date::date_time::parity_zone;

/// 对齐 Java: `cn.hutool.core.date.ZoneUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct ZoneUtil;

impl ZoneUtil {
    /// 默认 parity 时区 +08:00。
    pub fn to_default() -> FixedOffset {
        parity_zone()
    }

    /// 按时区 ID 粗粒度支持（Asia/Shanghai → +08）。
    pub fn to_zone(id: &str) -> FixedOffset {
        if id.contains("Shanghai") || id.contains("Chongqing") || id.ends_with("+08:00") {
            parity_zone()
        } else if id == "UTC" || id == "Z" || id == "GMT" {
            FixedOffset::east_opt(0).unwrap()
        } else {
            parity_zone()
        }
    }

    /// 兼容 sentinel。
    pub fn sentinel() -> Result<()> {
        Ok(())
    }
}
