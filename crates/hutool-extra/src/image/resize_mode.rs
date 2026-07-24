//! Bounded image decoding, resizing, cropping, and encoding.
//!
//! 对齐: `cn.hutool.core.img.ImgUtil`（extra 侧字节流语义）
//! Hutool ImgUtil 在 core；本模块提供同名门面，委托有界 resize/crop/convert。

use std::io::Cursor;

use ::image::{DynamicImage, GenericImageView, ImageFormat, ImageReader, imageops::FilterType};

use crate::{ExtraError, Result};

use super::image_limits::ImageLimits;

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

pub(crate) fn crop(

pub(crate) fn resize(

pub(crate) fn dimensions(bytes: &[u8], limits: ImageLimits) -> Result<(u32, u32)> {
    let image = decode(bytes, limits)?;
    Ok(image.dimensions())
}
