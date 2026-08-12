//! 对齐: format::date_printer
use crate::Result;
#[derive(Debug, Clone, Copy, Default)]
/// 对齐 Java: `cn.hutool.core.date.format.DatePrinter`。
pub struct DatePrinter;
impl DatePrinter {
    /// 兼容 sentinel。
    pub fn sentinel() -> Result<()> {
        Ok(())
    }
}
