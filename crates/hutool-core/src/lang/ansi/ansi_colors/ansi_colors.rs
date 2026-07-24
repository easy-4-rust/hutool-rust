//! 对齐: `cn.hutool.core.lang.ansi.AnsiColors`

use super::bit_depth::BitDepth;
use super::closest_color::ClosestColor;

/// 对齐 Java: `AnsiColors`
pub struct AnsiColors {
    depth: BitDepth,
}

impl AnsiColors {
    /// 构造
    pub fn new(depth: BitDepth) -> Self {
        Self { depth }
    }

    /// 找最近颜色（简化：灰度映射）
    pub fn find_closest(&self, r: u8, g: u8, b: u8) -> ClosestColor {
        let gray = ((r as u16 + g as u16 + b as u16) / 3) as u8;
        let code = match self.depth {
            BitDepth::Four => 30 + (gray / 36).min(7),
            BitDepth::Eight => 16 + (gray / 11).min(23),
        };
        ClosestColor { code }
    }
}
