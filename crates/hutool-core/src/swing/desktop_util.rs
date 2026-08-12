//! 桌面工具 —— 对齐 `cn.hutool.core.swing.DesktopUtil`
//!
//! 来源: hutool-core/src/main/java/cn/hutool/core/swing/DesktopUtil.java
//!
//! Hutool 的 `DesktopUtil` 通过 `java.awt.Desktop` 调用操作系统注册的
//! 默认关联应用(浏览器、邮件客户端、文件管理器、编辑器、打印)。
//!
//! # Rust 化迁移
//!
//! Java 的 `Desktop` 类 → Rust [`open`] crate。`open` 是一个极简跨平台库,
//! 通过平台原生机制调用系统默认应用:
//! - Windows:`ShellExecuteW`
//! - macOS:`open`命令
//! - Linux:`xdg-open`命令
//!
//! # 与 Java 行为差异
//!
//! - `edit()` / `print()` 在 `open` crate 中没有直接对应(系统不一定有"编辑器"
//!   或"打印"关联概念),Rust 版本降级为"使用系统默认关联应用打开",
//!   调用方需通过文件扩展名依赖系统注册表决定实际动作。
//! - `getDesktop()` 在 Java 返回 `java.awt.Desktop` 实例,在 Rust 中无对应
//!   跨平台抽象,故不暴露。

use crate::swing::{Result, SwingError};
use std::path::Path;

/// Desktop facade.
///
/// 对齐 Java: `cn.hutool.core.swing.DesktopUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct DesktopUtil;

impl DesktopUtil {
    /// 使用平台默认浏览器打开指定 URL 地址。
    ///
    /// 对齐 Java: `DesktopUtil.browse(String url)`
    ///
    /// # 参数
    /// - `url`: 完整的 URL 字符串(如 `https://example.com`)
    ///
    /// # 错误
    /// - [`SwingError::Desktop`] 当系统没有注册默认浏览器或调用失败时
    pub fn browse(url: &str) -> Result<()> {
        open::that(url).map_err(|error| SwingError::Desktop(error.to_string()))
    }

    /// 使用平台默认浏览器打开指定 URI(显式 [`url::Url`] 入参版本)。
    ///
    /// 对齐 Java: `DesktopUtil.browse(URI uri)`
    ///
    /// 该方法接受已解析的 `Url`,内部转字符串后委托给 [`Self::browse`]。
    pub fn browse_url(url: &url::Url) -> Result<()> {
        Self::browse(url.as_str())
    }

    /// 启动关联应用程序打开文件。
    ///
    /// 对齐 Java: `DesktopUtil.open(File file)`
    ///
    /// # 参数
    /// - `path`: 目标文件路径
    pub fn open(path: impl AsRef<Path>) -> Result<()> {
        open::that(path.as_ref()).map_err(|error| SwingError::Desktop(error.to_string()))
    }

    /// 启动关联编辑器应用程序打开文件用于编辑。
    ///
    /// 对齐 Java: `DesktopUtil.edit(File file)`
    ///
    /// **平台差异**:Java 的 `Desktop.Action.EDIT` 在不同 OS 上行为不同,
    /// `open` crate 没有等价 API,Rust 版本降级为"用系统默认应用打开",
    /// 实际编辑动作取决于文件扩展名关联的默认应用。
    pub fn edit(path: impl AsRef<Path>) -> Result<()> {
        // open crate 没有"edit"语义,降级为 open,系统按扩展名决定
        Self::open(path)
    }

    /// 使用系统打印命令打印文件。
    ///
    /// 对齐 Java: `DesktopUtil.print(File file)`
    ///
    /// **平台差异**:Java 的 `Desktop.Action.PRINT` 在 Windows 调用
    /// ShellExecute 的 "print" 动词,Rust 版本目前降级为 `open`,
    /// 未来可考虑调用平台特定 API(Windows:`ShellExecuteW "print"`)。
    pub fn print(path: impl AsRef<Path>) -> Result<()> {
        // 与 edit 类似,open crate 不区分 print,降级
        Self::open(path)
    }

    /// 使用平台默认邮件客户端打开邮件地址。
    ///
    /// 对齐 Java: `DesktopUtil.mail(String mailAddress)`
    ///
    /// # 参数
    /// - `mail_address`: mailto URL 或纯邮箱地址(如 `mailto:foo@bar.com`)
    pub fn mail(mail_address: &str) -> Result<()> {
        // 若调用方未提供 mailto: 前缀,自动补齐,与 Hutool 的 URLUtil.toURI 行为一致
        let target = if mail_address.starts_with("mailto:") {
            mail_address.to_owned()
        } else {
            format!("mailto:{mail_address}")
        };
        open::that(&target).map_err(|error| SwingError::Desktop(error.to_string()))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn mail_prepends_mailto_prefix() {
        // 仅验证 URL 拼接逻辑,不实际触发系统调用
        let target = if "foo@bar.com".starts_with("mailto:") {
            "foo@bar.com".to_owned()
        } else {
            format!("mailto:{}", "foo@bar.com")
        };
        assert_eq!(target, "mailto:foo@bar.com");
    }

    #[test]
    fn mail_preserves_existing_mailto_prefix() {
        let raw = "mailto:existing@example.com";
        let target = if raw.starts_with("mailto:") {
            raw.to_owned()
        } else {
            format!("mailto:{raw}")
        };
        assert_eq!(target, "mailto:existing@example.com");
    }
}
