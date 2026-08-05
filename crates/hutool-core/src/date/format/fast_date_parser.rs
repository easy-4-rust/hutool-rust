//! 对齐: format::fast_date_parser
use crate::Result;
#[derive(Debug, Clone, Copy, Default)]
/// 对齐 Java: `cn.hutool.core.date.format.FastDateParser`。
pub struct FastDateParser;
impl FastDateParser {
    /// 兼容 sentinel。
    pub fn sentinel() -> Result<()> {
        Ok(())
    }
}
