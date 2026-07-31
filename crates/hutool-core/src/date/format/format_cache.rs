//! 对齐: format::format_cache
use crate::Result;
#[derive(Debug, Clone, Copy, Default)]
/// 对齐 Java: `cn.hutool.core.date.format.FastDateFormat` 格式缓存。
pub struct FormatCache;
impl FormatCache {
    /// 兼容 sentinel。
    pub fn sentinel() -> Result<()> { Ok(()) }
}
