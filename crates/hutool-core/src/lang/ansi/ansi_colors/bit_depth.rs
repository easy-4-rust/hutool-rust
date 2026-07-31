//! 对齐: `cn.hutool.core.lang.ansi.AnsiColors`

#![allow(dead_code)] // 对齐 Java AnsiColors，暂未接线，预留

/// 色深
#[derive(Debug, Clone, Copy)]
pub enum BitDepth {
    /// 4-bit
    Four,
    /// 8-bit
    Eight,
}
