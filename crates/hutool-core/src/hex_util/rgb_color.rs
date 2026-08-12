//! Convenient hexadecimal operations aligned with Hutool's `HexUtil` family.

/// 对齐: `cn.hutool.core.util.HexUtil.RGBColor`
/// RGB 颜色

/// An RGB color whose channels are always in the valid `0..=255` range.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RgbColor {
    /// Red channel.
    pub red: u8,
    /// Green channel.
    pub green: u8,
    /// Blue channel.
    pub blue: u8,
}

impl RgbColor {
    /// Creates a color from its red, green, and blue channels.
    #[must_use]
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
}
