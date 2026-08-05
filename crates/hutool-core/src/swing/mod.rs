//! `cn.hutool.core.swing` 子包对齐实现
//!
//! 对齐 Hutool 的 `cn.hutool.core.swing` 子包(3 个 Java 类,68 个公共 API),
//! 使用 Rust 跨平台 GUI 生态组件替代 Java AWT/Swing:
//!
//! - [`desktop_util`] → `cn.hutool.core.swing.DesktopUtil`
//!   底层:[`open`] crate —— 跨平台调用系统默认浏览器/邮件客户端/文件管理器
//!
//! - [`screen_util`] → `cn.hutool.core.swing.ScreenUtil`
//!   底层:[`xcap`] crate —— 跨平台屏幕尺寸查询与屏幕捕获
//!   (Windows: DXGI/Direct3D;macOS: CGWindowListCreateImage;Linux: X11/XCB)
//!
//! - [`robot_util`] → `cn.hutool.core.swing.RobotUtil`
//!   底层:[`enigo`] crate —— 跨平台键鼠模拟
//!   (Windows: SendInput;macOS: CGEventPost;Linux: XTestFakeKeyEvent)
//!
//! - [`clipboard`] → `cn.hutool.core.swing.clipboard.*`
//!   子包对齐桩(5 个 Java 类)
//!
//! # Feature Gate
//!
//! 该模块仅当启用 `swing` feature 时编译,因为它依赖平台特定的 GUI 子系统。
//! 在 headless 环境(CI/服务器)上启用可能失败。
//!
//! # Rust 化要点
//!
//! - Java `Robot`(状态ful 单例) → Rust `RobotUtil`(持有 `enigo::Enigo` 句柄)
//! - Java `BufferedImage` → Rust `image::RgbaImage`(xcap 原生产物)
//! - Java `Dimension` / `Rectangle` → Rust `(u32, u32)` / [`ScreenRect`]
//! - Java `KeyEvent.VK_*` 常量 → Rust `enigo::Key` 枚举
//! - Java `InputEvent.BUTTON1_MASK` → Rust [`MouseButton::Left`]
//! - 异常:`IORuntimeException` / `UtilException` → Rust `Result<_, SwingError>`

pub mod clipboard;
pub mod desktop_util;
pub mod robot_util;
pub mod screen_util;

pub use desktop_util::DesktopUtil;
pub use robot_util::{MouseButton, RobotError, RobotUtil};
pub use screen_util::{ScreenError, ScreenRect, ScreenUtil};

/// Swing 子包统一错误类型别名。
///
/// 对齐 Java: `cn.hutool.core.exceptions.UtilException` 在 swing 路径上的包装。
pub type Result<T> = std::result::Result<T, SwingError>;

/// Swing 子包错误枚举。
///
/// 对齐 Java: `cn.hutool.exceptions.UtilException` + `IORuntimeException`
/// 在 swing 路径上的合并表达。
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum SwingError {
    /// 调用系统默认应用失败(浏览器/邮件/文件管理器)。
    /// 对齐 Java: `DesktopUtil.browse/open/mail` 抛出的 `IOException`。
    #[error("desktop operation failed: {0}")]
    Desktop(String),
    /// 屏幕尺寸查询或截屏失败。
    /// 对齐 Java: `ScreenUtil.captureScreen` 抛出的 `AWTException`。
    #[error("screen capture failed: {0}")]
    Screen(String),
    /// 键鼠模拟失败。
    /// 对齐 Java: `RobotUtil` 路径抛出的 `AWTException`。
    #[error("robot simulation failed: {0}")]
    Robot(String),
    /// 图像编解码失败。
    /// 对齐 Java: `ImgUtil.write` 抛出的 `IORuntimeException`。
    #[error("image codec failed: {0}")]
    Image(String),
    /// 文件 IO 失败。
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
