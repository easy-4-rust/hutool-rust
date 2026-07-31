//! 对齐: `cn.hutool.core.lang.ansi.AnsiEncoder`

#![allow(dead_code)] // 对齐 Java AnsiEncoder，暂未接线，预留

use crate::lang::ansi::ansi_color::AnsiColor;

/// 对齐 Java: `AnsiEncoder`
pub struct AnsiEncoder;

impl AnsiEncoder {
    /// 对齐 `AnsiEncoder.encode(AnsiElement..., CharSequence)`
    pub fn encode(color: AnsiColor, text: &str) -> String {
        format!("\u{001B}[{}m{}\u{001B}[0;39m", color.code(), text)
    }

    /// 前景+背景
    pub fn encode_fore_back(fore_code: u8, back_code: u8, text: &str) -> String {
        format!("\u{001B}[{};{}m{}\u{001B}[0;39m", fore_code, back_code, text)
    }
}
