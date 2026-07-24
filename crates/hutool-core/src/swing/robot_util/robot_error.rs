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

use super::mouse_button::MouseButton;
use super::robot_util::RobotUtil;

/// Robot 错误别名。
///
/// 对齐 Java: `RobotUtil` 路径抛出的 `AWTException`
pub type RobotError = SwingError;

impl RobotUtil {
    /// 创建一个 Enigo 句柄。
    fn enigo() -> Result<Enigo> {
        Enigo::new(&Settings::default()).map_err(|error| SwingError::Robot(error.to_string()))
    }

    /// 设置默认的延迟时间(毫秒)。
    ///
    /// 对齐 Java: `RobotUtil.setDelay(int delayMillis)`
    ///
    /// 当按键执行完后的等待时间,也可以用 `ThreadUtil::sleep` 方法代替。
    pub fn set_delay(delay_millis: u64) {
        DEFAULT_DELAY_MILLIS.store(delay_millis, std::sync::atomic::Ordering::Relaxed);
    }

    /// 获取全局默认的延迟时间(毫秒)。
    ///
    /// 对齐 Java: `RobotUtil.getDelay()`
    pub fn get_delay() -> u64 {
        DEFAULT_DELAY_MILLIS.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// 按 setDelay 设置的全局延迟等待。
    ///
    /// 对齐 Java: `RobotUtil.delay()`
    fn delay_if_set() {
        let millis = Self::get_delay();
        if millis > 0 {
            std::thread::sleep(Duration::from_millis(millis));
        }
    }

    /// 模拟鼠标移动到指定坐标。
    ///
    /// 对齐 Java: `RobotUtil.mouseMove(int x, int y)`
    pub fn mouse_move(x: i32, y: i32) -> Result<()> {
        let mut enigo = Self::enigo()?;
        enigo
            .move_mouse(x, y, enigo::Coordinate::Abs)
            .map_err(|error| SwingError::Robot(error.to_string()))?;
        Self::delay_if_set();
        Ok(())
    }

    /// 模拟鼠标单击(默认左键)。
    ///
    /// 对齐 Java: `RobotUtil.click()`
    pub fn click() -> Result<()> {
        Self::click_button(MouseButton::Left)
    }

    /// 模拟鼠标右键单击。
    ///
    /// 对齐 Java: `RobotUtil.rightClick()`
    pub fn right_click() -> Result<()> {
        Self::click_button(MouseButton::Right)
    }

    /// 模拟鼠标指定按键单击(底层通用实现)。
    ///
    /// 对齐 Java: `RobotUtil` 内部 `ROBOT.mousePress/mouseRelease` 序列。
    pub fn click_button(button: MouseButton) -> Result<()> {
        let mut enigo = Self::enigo()?;
        let enigo_button = button.to_enigo();
        enigo
            .button(enigo_button, Direction::Press)
            .map_err(|error| SwingError::Robot(error.to_string()))?;
        enigo
            .button(enigo_button, Direction::Release)
            .map_err(|error| SwingError::Robot(error.to_string()))?;
        Self::delay_if_set();
        Ok(())
    }

    /// 模拟鼠标滚轮滚动。
    ///
    /// 对齐 Java: `RobotUtil.mouseWheel(int wheelAmt)`
    ///
    /// # 参数
    /// - `wheel_amt`:滚动数,负数表示向前滚动,正数向后滚动
    pub fn mouse_wheel(wheel_amt: i32) -> Result<()> {
        let mut enigo = Self::enigo()?;
        // enigo 0.2 的 scroll 约定:正值向上滚动,Axis::Horizontal/Vertical
        // Java 的 wheelAmt 负数向前(向上),正数向后(向下),故取反
        let amt = -wheel_amt;
        enigo
            .scroll(amt, enigo::Axis::Vertical)
            .map_err(|error| SwingError::Robot(error.to_string()))?;
        Self::delay_if_set();
        Ok(())
    }

    /// 模拟键盘点击(包括按下和释放)。
    ///
    /// 对齐 Java: `RobotUtil.keyClick(int... keyCodes)`
    ///
    /// # 参数
    /// - `keys`:按键列表(见 [`enigo::Key`] 枚举)
    pub fn key_click(keys: &[Key]) -> Result<()> {
        let mut enigo = Self::enigo()?;
        for key in keys {
            enigo
                .key(*key, Direction::Press)
                .map_err(|error| SwingError::Robot(error.to_string()))?;
            enigo
                .key(*key, Direction::Release)
                .map_err(|error| SwingError::Robot(error.to_string()))?;
        }
        Self::delay_if_set();
        Ok(())
    }

    /// 打印输出指定字符串。
    ///
    /// 对齐 Java: `RobotUtil.keyPressString(String str)`
    ///
    /// **平台差异**:Java 通过剪贴板 + Ctrl+V 实现,Rust 版本直接用 enigo 的
    /// `text()` 逐字符输入,无需剪贴板依赖。
    pub fn key_press_string(text: &str) -> Result<()> {
        let mut enigo = Self::enigo()?;
        enigo
            .text(text)
            .map_err(|error| SwingError::Robot(error.to_string()))?;
        Self::delay_if_set();
        Ok(())
    }

    /// 模拟 Shift + 按键。
    ///
    /// 对齐 Java: `RobotUtil.keyPressWithShift(int key)`
    pub fn key_press_with_shift(key: Key) -> Result<()> {
        Self::key_with_modifier(Key::Shift, key)
    }

    /// 模拟 Ctrl + 按键。
    ///
    /// 对齐 Java: `RobotUtil.keyPressWithCtrl(int key)`
    pub fn key_press_with_ctrl(key: Key) -> Result<()> {
        Self::key_with_modifier(Key::Control, key)
    }

    /// 模拟 Alt + 按键。
    ///
    /// 对齐 Java: `RobotUtil.keyPressWithAlt(int key)`
    pub fn key_press_with_alt(key: Key) -> Result<()> {
        Self::key_with_modifier(Key::Alt, key)
    }

    /// 通用"修饰键 + 普通键"组合实现。
    fn key_with_modifier(modifier: Key, key: Key) -> Result<()> {
        let mut enigo = Self::enigo()?;
        enigo
            .key(modifier, Direction::Press)
            .map_err(|error| SwingError::Robot(error.to_string()))?;
        enigo
            .key(key, Direction::Press)
            .map_err(|error| SwingError::Robot(error.to_string()))?;
        enigo
            .key(key, Direction::Release)
            .map_err(|error| SwingError::Robot(error.to_string()))?;
        enigo
            .key(modifier, Direction::Release)
            .map_err(|error| SwingError::Robot(error.to_string()))?;
        Self::delay_if_set();
        Ok(())
    }

    /// 截取全屏。
    ///
    /// 对齐 Java: `RobotUtil.captureScreen()` → `BufferedImage`
    pub fn capture_screen() -> Result<RgbaImage> {
        ScreenUtil::capture_screen()
    }

    /// 截取全屏到文件。
    ///
    /// 对齐 Java: `RobotUtil.captureScreen(File outFile)`
    pub fn capture_screen_to_file(out_file: impl AsRef<Path>) -> Result<()> {
        ScreenUtil::capture_screen_to_file(out_file)
    }

    /// 截取指定矩形区域。
    ///
    /// 对齐 Java: `RobotUtil.captureScreen(Rectangle screenRect)` → `BufferedImage`
    pub fn capture_screen_region(rect: ScreenRect) -> Result<RgbaImage> {
        ScreenUtil::capture_screen_region(rect)
    }

    /// 截取指定矩形区域到文件。
    ///
    /// 对齐 Java: `RobotUtil.captureScreen(Rectangle screenRect, File outFile)`
    pub fn capture_screen_region_to_file(rect: ScreenRect, out_file: impl AsRef<Path>) -> Result<()> {
        ScreenUtil::capture_screen_region_to_file(rect, out_file)
    }
}

static DEFAULT_DELAY_MILLIS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
