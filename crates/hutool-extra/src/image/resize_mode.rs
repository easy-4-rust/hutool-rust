//! Bounded image decoding, resizing, cropping, and encoding.
//!
//! 对齐: `cn.hutool.core.img.ImgUtil`（extra 侧字节流语义）
//! Hutool ImgUtil 在 core；本模块提供同名门面，委托有界 resize/crop/convert。

use std::io::Cursor;

use ::image::{DynamicImage, GenericImageView, ImageFormat, ImageReader, imageops::FilterType};

use crate::{ExtraError, Result};

use super::image_limits::ImageLimits;
use super::output_format::OutputFormat;

/// Geometry strategy used by [`resize`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResizeMode {
    /// Preserve aspect ratio and fit entirely inside the target box.
    Fit,
    /// Preserve aspect ratio and crop overflow to fill the target box.
    FillCrop,
    /// Ignore aspect ratio and force exact dimensions.
    Exact,
}

/// Decodes image bytes, enforcing the given defensive limits.
pub(crate) fn decode(bytes: &[u8], limits: ImageLimits) -> Result<DynamicImage> {
    if bytes.len() > limits.max_input_bytes {
        return Err(ExtraError::ImageLimit("encoded input bytes"));
    }
    let image = ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()
        .map_err(ExtraError::Io)?
        .decode()
        .map_err(ExtraError::Image)?;
    let (width, height) = image.dimensions();
    if width > limits.max_width || height > limits.max_height {
        return Err(ExtraError::ImageLimit("decoded dimensions"));
    }
    if u64::from(width) * u64::from(height) > limits.max_pixels {
        return Err(ExtraError::ImageLimit("decoded pixels"));
    }
    Ok(image)
}

/// Returns the decoded image width and height.
pub fn dimensions(bytes: &[u8], limits: ImageLimits) -> Result<(u32, u32)> {
    let image = decode(bytes, limits)?;
    Ok(image.dimensions())
}

/// Decodes, resizes according to the given [`ResizeMode`], and re-encodes.
pub fn resize(
    bytes: &[u8],
    width: u32,
    height: u32,
    mode: ResizeMode,
    output: OutputFormat,
    limits: ImageLimits,
) -> Result<Vec<u8>> {
    let image = decode(bytes, limits)?;
    match mode {
        ResizeMode::Exact => {
            let resized = image.resize_exact(width.max(1), height.max(1), FilterType::Lanczos3);
            encode(&resized, output, limits)
        }
        ResizeMode::Fit => {
            let (src_w, src_h) = image.dimensions();
            let scale = (f64::from(width) / f64::from(src_w))
                .min(f64::from(height) / f64::from(src_h))
                .max(0.0);
            let target_w = (f64::from(src_w) * scale).round().max(1.0) as u32;
            let target_h = (f64::from(src_h) * scale).round().max(1.0) as u32;
            let resized = image.resize_exact(target_w, target_h, FilterType::Lanczos3);
            encode(&resized, output, limits)
        }
        ResizeMode::FillCrop => {
            let (src_w, src_h) = image.dimensions();
            let scale = (f64::from(width) / f64::from(src_w))
                .max(f64::from(height) / f64::from(src_h))
                .max(0.0);
            let scaled_w = (f64::from(src_w) * scale).round().max(1.0) as u32;
            let scaled_h = (f64::from(src_h) * scale).round().max(1.0) as u32;
            let resized = image.resize_exact(scaled_w, scaled_h, FilterType::Lanczos3);
            let crop_x = (scaled_w.saturating_sub(width)) / 2;
            let crop_y = (scaled_h.saturating_sub(height)) / 2;
            let cropped = resized.crop_imm(
                crop_x,
                crop_y,
                scaled_w.min(width).max(1),
                scaled_h.min(height).max(1),
            );
            encode(&cropped, output, limits)
        }
    }
}

/// Decodes, crops a rectangle, and re-encodes.
pub fn crop(
    bytes: &[u8],
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    output: OutputFormat,
    limits: ImageLimits,
) -> Result<Vec<u8>> {
    let image = decode(bytes, limits)?;
    let cropped = image.crop_imm(x, y, width.max(1), height.max(1));
    encode(&cropped, output, limits)
}

fn encode(image: &DynamicImage, output: OutputFormat, limits: ImageLimits) -> Result<Vec<u8>> {
    let mut out = Vec::new();
    match output {
        OutputFormat::Png => image.write_to(&mut Cursor::new(&mut out), ImageFormat::Png),
        OutputFormat::WebP => image.write_to(&mut Cursor::new(&mut out), ImageFormat::WebP),
        OutputFormat::Jpeg(quality) => {
            let encoder = ::image::codecs::jpeg::JpegEncoder::new_with_quality(&mut out, quality);
            image.write_with_encoder(encoder)
        }
    }
    .map_err(ExtraError::Image)?;
    if out.len() > limits.max_output_bytes {
        return Err(ExtraError::ImageLimit("encoded output bytes"));
    }
    Ok(out)
}
