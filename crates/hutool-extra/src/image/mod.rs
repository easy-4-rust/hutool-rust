//! Bounded image decoding, resizing, cropping, and encoding.
//!
//! 对齐: `cn.hutool.core.img.ImgUtil`（extra 侧字节流语义）
//! Hutool ImgUtil 在 core；本模块提供同名门面，委托有界 resize/crop/convert。

mod image_limits;
mod img_util;
mod output_format;
mod resize_mode;

pub use image_limits::ImageLimits;
pub use img_util::ImgUtil;
pub use output_format::OutputFormat;
pub use resize_mode::ResizeMode;
pub use resize_mode::crop;
pub use resize_mode::dimensions;
pub use resize_mode::resize;
