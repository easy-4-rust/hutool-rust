//! 对齐: `cn.hutool.core.date.TimeZone`

use crate::Result;
use crate::date::zone_util::ZoneUtil;

/// 对齐 Java TimeZone 相关工具（委托 ZoneUtil）。
#[derive(Debug, Clone, Copy, Default)]
pub struct TimeZoneUtil;

impl TimeZoneUtil {
    /// 默认 +08。
    pub fn get_default() -> chrono::FixedOffset {
        ZoneUtil::to_default()
    }

    /// 兼容 sentinel。
    pub fn sentinel() -> Result<()> {
        Ok(())
    }
}
