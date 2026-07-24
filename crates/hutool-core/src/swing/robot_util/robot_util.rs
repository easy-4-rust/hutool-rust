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

/// Robot facade。
///
/// 对齐 Java: `cn.hutool.core.swing.RobotUtil`(Java 中为静态方法 + 静态 Robot 单例;
/// Rust 版本以 ZST + 关联函数呈现,内部每次操作创建 Enigo 句柄)。
///
/// # Rust 化说明
///
/// Java 的 `Robot` 在静态块中创建单例,所有静态方法共享。Rust 的
/// [`enigo::Enigo`] 不是 `Sync`,无法简单放进 `static`。该实现采用
/// "每次操作 new 一个 Enigo"的简化策略,调用方在批量操作时应考虑
/// 自行持有 `Enigo` 实例以减少开销。
#[derive(Debug, Clone, Copy, Default)]
pub struct RobotUtil;
