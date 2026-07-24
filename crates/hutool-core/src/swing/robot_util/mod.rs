//! 键鼠模拟工具 —— 对齐 `cn.hutool.core.swing.RobotUtil`
//!
//! 来源: hutool-core/src/main/java/cn/hutool/core/swing/RobotUtil.java
//!
//! Hutool 的 `RobotUtil` 通过 `java.awt.Robot` 模拟键鼠操作和屏幕截图。
//!
//! # Rust 化迁移
//!
//! Java 的 `java.awt.Robot` 单例 → Rust [`RobotUtil`] 持有
//! [`enigo::Enigo`] 句柄(enigo 在 Windows 用 SendInput、macOS 用
//! CGEventPost、Linux 用 XTest)。
//!
//! # 与 Java 行为差异
//!
//! - Java 的 `keyPressString(String)` 通过剪贴板 + Ctrl+V 实现,Rust 版本
//!   直接用 enigo 的 `text()` 逐字符输入,无需剪贴板依赖。
//! - Java 的 `keyPress(KeyEvent.VK_*)` 接受 int 虚拟键码,Rust 版本
//!   接受 [`enigo::Key`] 枚举,与平台无关。
//! - Java 的 `delay()` 在 RobotUtil 内部用 `Thread.sleep`,Rust 版本
//!   用 `std::thread::sleep`。
//! - Java 静态方法 → Rust ZST + 关联函数,enigo 句柄通过
//!   `thread_local!` 或每次 new 创建(简化起见用每次 new)。

use crate::swing::screen_util::{ScreenRect, ScreenUtil};
use crate::swing::{Result, SwingError};
use enigo::{Direction, Enigo, Key, Keyboard, Mouse, Settings};
use image::RgbaImage;
use std::path::Path;
use std::time::Duration;

mod mouse_button;
mod robot_util;
mod robot_error;

pub use mouse_button::MouseButton;
pub use robot_util::RobotUtil;
pub use robot_error::RobotError;
