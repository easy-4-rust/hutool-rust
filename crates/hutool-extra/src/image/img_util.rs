//! Bounded image decoding, resizing, cropping, and encoding.
//!
//! 对齐: `cn.hutool.core.img.ImgUtil`（extra 侧字节流语义）
//! Hutool `ImgUtil` 在 core；本模块提供同名门面，委托有界 resize/crop/convert。

use crate::{ExtraError, Result};

use super::image_limits::ImageLimits;
use super::output_format::OutputFormat;
use super::resize_mode::{ResizeMode, crop, dimensions, resize};

/// Hutool-named image facade over bounded byte-oriented helpers.
///
/// 对齐 Java 类: `cn.hutool.core.img.ImgUtil`（字节流子集；无 AWT `Image`）
pub struct ImgUtil;

impl ImgUtil {
    /// PNG type constant (Hutool `IMAGE_TYPE_PNG`).
    pub const IMAGE_TYPE_PNG: &'static str = "png";
    /// JPEG type constant (Hutool `IMAGE_TYPE_JPG`).
    pub const IMAGE_TYPE_JPG: &'static str = "jpg";
    /// JPEG type constant (Hutool `IMAGE_TYPE_JPEG`).
    pub const IMAGE_TYPE_JPEG: &'static str = "jpeg";

    /// Returns width/height (Hutool `readSize` / dimension helpers).
    pub fn read_size(bytes: &[u8]) -> Result<(u32, u32)> {
        dimensions(bytes, ImageLimits::default())
    }

    /// Scales into a target box preserving aspect ratio (Hutool `scale(width, height)`).
    pub fn scale(bytes: &[u8], width: u32, height: u32, output: OutputFormat) -> Result<Vec<u8>> {
        resize(
            bytes,
            width,
            height,
            ResizeMode::Fit,
            output,
            ImageLimits::default(),
        )
    }

    /// Scales by a positive factor (Hutool `scale(float)`).
    pub fn scale_by(bytes: &[u8], scale: f32, output: OutputFormat) -> Result<Vec<u8>> {
        if !(scale.is_finite() && scale > 0.0) {
            return Err(ExtraError::ImageLimit("scale factor"));
        }
        let (width, height) = dimensions(bytes, ImageLimits::default())?;
        let target_w = ((f64::from(width) * f64::from(scale)).round() as u32).max(1);
        let target_h = ((f64::from(height) * f64::from(scale)).round() as u32).max(1);
        resize(
            bytes,
            target_w,
            target_h,
            ResizeMode::Exact,
            output,
            ImageLimits::default(),
        )
    }

    /// Crops a rectangle (Hutool `cut`).
    pub fn cut(
        bytes: &[u8],
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        output: OutputFormat,
    ) -> Result<Vec<u8>> {
        crop(bytes, x, y, width, height, output, ImageLimits::default())
    }

    /// Converts between encoded formats (Hutool `convert`).
    pub fn convert(bytes: &[u8], format_name: &str) -> Result<Vec<u8>> {
        let (width, height) = dimensions(bytes, ImageLimits::default())?;
        let output = output_from_name(format_name, 90)?;
        resize(
            bytes,
            width,
            height,
            ResizeMode::Exact,
            output,
            ImageLimits::default(),
        )
    }

    /// Re-encodes as JPEG with quality (Hutool compress-quality path).
    pub fn compress(bytes: &[u8], quality: u8) -> Result<Vec<u8>> {
        let (width, height) = dimensions(bytes, ImageLimits::default())?;
        resize(
            bytes,
            width,
            height,
            ResizeMode::Exact,
            OutputFormat::Jpeg(quality),
            ImageLimits::default(),
        )
    }
}

fn output_from_name(format: &str, jpeg_quality: u8) -> Result<OutputFormat> {
    match format.trim().to_ascii_lowercase().as_str() {
        "png" => Ok(OutputFormat::Png),
        "jpg" | "jpeg" => Ok(OutputFormat::Jpeg(jpeg_quality)),
        "webp" => Ok(OutputFormat::WebP),
        _ => Err(ExtraError::ImageLimit("unsupported output format")),
    }
}
