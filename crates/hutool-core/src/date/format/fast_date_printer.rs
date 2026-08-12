//! 对齐: format::fast_date_printer
use crate::Result;
#[derive(Debug, Clone, Copy, Default)]
/// 对齐 Java: `cn.hutool.core.date.format.FastDatePrinter`。
pub struct FastDatePrinter;
impl FastDatePrinter {
    /// 兼容 sentinel。
    pub fn sentinel() -> Result<()> {
        Ok(())
    }
}
