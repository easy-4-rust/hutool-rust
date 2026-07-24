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

use super::screen_util::ScreenUtil;

/// 屏幕矩形区域,对应 Java `java.awt.Rectangle`。
///
/// 对齐 Java: `java.awt.Rectangle` 在 ScreenUtil/RobotUtil 路径中的用法
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScreenRect {
    /// 矩形左上角 X 坐标(像素,相对于显示器虚拟坐标)。
    pub x: i32,
    /// 矩形左上角 Y 坐标(像素,相对于显示器虚拟坐标)。
    pub y: i32,
    /// 矩形宽度(像素)。
    pub width: u32,
    /// 矩形高度(像素)。
    pub height: u32,
}

impl ScreenRect {
    /// 创建一个从原点开始的指定宽高的矩形。
    ///
    /// 对齐 Java: `new Rectangle(int width, int height)`
    #[must_use]
    pub const fn new(width: u32, height: u32) -> Self {
        Self {
            x: 0,
            y: 0,
            width,
            height,
        }
    }

    /// 创建一个指定位置和尺寸的矩形。
    ///
    /// 对齐 Java: `new Rectangle(int x, int y, int width, int height)`
    #[must_use]
    pub const fn with_origin(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}
