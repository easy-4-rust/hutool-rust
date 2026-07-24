//! 屏幕工具 —— 对齐 `cn.hutool.core.swing.ScreenUtil`
//!
//! 来源: hutool-core/src/main/java/cn/hutool/core/swing/ScreenUtil.java
//!
//! Hutool 的 `ScreenUtil` 通过 `java.awt.Toolkit` 查询屏幕尺寸,
//! 并通过 `Robot.createScreenCapture` 进行屏幕截图。
//!
//! # Rust 化迁移
//!
//! Java 的 `Toolkit.getDefaultToolkit().getScreenSize()` → Rust [`xcap::Monitor`]
//! (跨平台屏幕枚举与捕获)。
//!
//! Java 的 `BufferedImage` → Rust `image::RgbaImage`(xcap 的原生产物)。
//!
//! Java 的 `java.awt.Rectangle` → Rust [`ScreenRect`]。
//!
//! # 多显示器注意
//!
//! Hutool 只返回"主屏幕"信息,`xcap` 提供所有显示器枚举,Rust 版本通过
//! [`ScreenUtil::primary_monitor`] 选择主显示器,其余显示器可由调用方自行枚举。

use crate::swing::{Result, SwingError};
use image::RgbaImage;
use std::path::Path;

mod screen_rect;
mod screen_util;
mod screen_error;

pub use screen_rect::ScreenRect;
pub use screen_util::ScreenUtil;
pub use screen_error::ScreenError;
