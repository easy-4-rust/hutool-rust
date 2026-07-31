//! 对齐: format::abstract_date_basic
use crate::Result;
#[derive(Debug, Clone, Copy, Default)]
/// 对齐 Java: `cn.hutool.core.date.format.AbstractDateBasic`。
pub struct AbstractDateBasic;
impl AbstractDateBasic {
    /// 兼容 sentinel。
    pub fn sentinel() -> Result<()> { Ok(()) }
}
