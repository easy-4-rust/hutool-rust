//! 对齐: format::date_basic
use crate::Result;
#[derive(Debug, Clone, Copy, Default)]
/// 对齐 Java: `cn.hutool.core.date.format.DateBasic`。
pub struct DateBasic;
impl DateBasic {
    /// 兼容 sentinel。
    pub fn sentinel() -> Result<()> {
        Ok(())
    }
}
