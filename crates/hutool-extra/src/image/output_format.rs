//! Bounded image decoding, resizing, cropping, and encoding.
//!
//! 对齐: `cn.hutool.core.img.ImgUtil`（extra 侧字节流语义）
//! Hutool ImgUtil 在 core；本模块提供同名门面，委托有界 resize/crop/convert。

/// Encoded image output format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    /// Portable Network Graphics.
    Png,
    /// JPEG with quality from 1 through 100.
    Jpeg(u8),
    /// Lossless WebP.
    WebP,
}
