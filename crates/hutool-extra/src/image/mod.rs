//! Bounded image decoding, resizing, cropping, and encoding.
//!
//! 对齐: `cn.hutool.core.img.ImgUtil`（extra 侧字节流语义）
//! Hutool ImgUtil 在 core；本模块提供同名门面，委托有界 resize/crop/convert。

use std::io::Cursor;

use ::image::{DynamicImage, GenericImageView, ImageFormat, ImageReader, imageops::FilterType};

use crate::{ExtraError, Result};

mod image_limits;
mod resize_mode;
mod output_format;
mod img_util;

pub use image_limits::ImageLimits;
pub use resize_mode::ResizeMode;
pub use output_format::OutputFormat;
pub use img_util::ImgUtil;
pub use resize_mode::dimensions;
pub use resize_mode::resize;
pub use resize_mode::crop;
