//! 对齐: `cn.hutool.core.lang.ansi.AnsiColors`

#![allow(dead_code)] // 对齐 Java AnsiColors，暂未接线，预留

/// 最近色结果
#[derive(Debug, Clone, Copy)]
pub struct ClosestColor {
    /// ANSI 码
    pub code: u8,
}

impl ClosestColor {
    /// 转为前景/背景码
    pub fn to_ansi_code(self, fore: bool) -> u8 {
        if fore {
            self.code
        } else {
            self.code.saturating_add(10)
        }
    }
}
