//! Bounded image decoding, resizing, cropping, and encoding.
//!
//! 对齐: `cn.hutool.core.img.ImgUtil`（extra 侧字节流语义）
//! Hutool ImgUtil 在 core；本模块提供同名门面，委托有界 resize/crop/convert。

use std::io::Cursor;

use ::image::{DynamicImage, GenericImageView, ImageFormat, ImageReader, imageops::FilterType};

use crate::{ExtraError, Result};

/// Defensive limits applied to encoded and decoded images.
#[derive(Debug, Clone, Copy)]
pub struct ImageLimits {
    /// Maximum encoded input bytes.
    pub max_input_bytes: usize,
    /// Maximum decoded width.
    pub max_width: u32,
    /// Maximum decoded height.
    pub max_height: u32,
    /// Maximum decoded pixel count.
    pub max_pixels: u64,
    /// Maximum encoded output bytes.
    pub max_output_bytes: usize,
}

impl Default for ImageLimits {
    fn default() -> Self {
        Self {
            max_input_bytes: 16 * 1024 * 1024,
            max_width: 8_192,
            max_height: 8_192,
            max_pixels: 40_000_000,
            max_output_bytes: 32 * 1024 * 1024,
        }
    }
}
