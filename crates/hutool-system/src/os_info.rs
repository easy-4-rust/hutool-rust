//! 对齐: `cn.hutool.system.OsInfo`
//! 来源: `hutool-system/src/main/java/cn/hutool/system/OsInfo.java`
//! 中文说明: 提供操作系统名称、版本、架构以及 Hutool 风格的系统家族判断谓词。

use std::{env, fmt};

use sysinfo::System;

#[cfg(windows)]
const LINE_SEPARATOR: &str = "\r\n";
#[cfg(not(windows))]
const LINE_SEPARATOR: &str = "\n";
#[cfg(windows)]
const PATH_SEPARATOR: char = ';';
#[cfg(not(windows))]
const PATH_SEPARATOR: char = ':';

/// 对齐: `cn.hutool.system.OsInfo`
/// 中文说明: 操作系统属性及 Hutool 兼容的系统家族判断谓词。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OsInfo {
    /// 中文说明: 目标架构。
    pub arch: String,
    /// 中文说明: 操作系统名称。
    pub name: String,
    /// 中文说明: 操作系统版本。
    pub version: String,
    /// 中文说明: 文件系统分隔符。
    pub file_separator: char,
    /// 中文说明: 原生行分隔符。
    pub line_separator: &'static str,
    /// 中文说明: 搜索路径分隔符。
    pub path_separator: char,
}

impl OsInfo {
    /// 中文说明: 从显式参数创建 OS 视图，适用于确定性策略检查。
    /// 对齐 Java 方法: `OsInfo` 构造逻辑
    #[must_use]
    pub fn from_parts(
        arch: impl Into<String>,
        name: impl Into<String>,
        version: impl Into<String>,
    ) -> Self {
        Self {
            arch: arch.into(),
            name: name.into(),
            version: version.into(),
            file_separator: std::path::MAIN_SEPARATOR,
            line_separator: LINE_SEPARATOR,
            path_separator: PATH_SEPARATOR,
        }
    }

    /// 中文说明: 采集当前操作系统属性。
    /// 对齐 Java 方法: `SystemUtil.getOsInfo`
    #[must_use]
    pub fn collect() -> Self {
        Self::from_parts(
            env::consts::ARCH,
            system_name_or_fallback(),
            System::os_version().unwrap_or_default(),
        )
    }

    fn normalized_name(&self) -> String {
        self.name.to_ascii_lowercase().replace([' ', '-'], "")
    }

    /// 中文说明: 判断操作系统名称是否匹配 AIX。
    /// 对齐 Java 方法: `SystemUtil.isAix`
    #[must_use]
    pub fn is_aix(&self) -> bool {
        self.normalized_name().contains("aix")
    }

    /// 中文说明: 判断操作系统名称是否匹配 HP-UX。
    /// 对齐 Java 方法: `SystemUtil.isHpUx`
    #[must_use]
    pub fn is_hp_ux(&self) -> bool {
        self.normalized_name().contains("hpux")
    }

    /// 中文说明: 判断操作系统名称是否匹配 IRIX。
    /// 对齐 Java 方法: `SystemUtil.isIrix`
    #[must_use]
    pub fn is_irix(&self) -> bool {
        self.normalized_name().contains("irix")
    }

    /// 中文说明: 判断操作系统名称是否匹配 Linux。
    /// 对齐 Java 方法: `SystemUtil.isLinux`
    #[must_use]
    pub fn is_linux(&self) -> bool {
        self.normalized_name().contains("linux")
    }

    /// 中文说明: 判断操作系统名称是否匹配 macOS。
    /// 对齐 Java 方法: `SystemUtil.isMac`
    #[must_use]
    pub fn is_mac(&self) -> bool {
        let name = self.normalized_name();
        name.contains("mac") || name.contains("darwin")
    }

    /// 中文说明: `is_mac` 的别名。
    /// 对齐 Java 方法: `SystemUtil.isMacOSX`
    #[must_use]
    pub fn is_mac_os_x(&self) -> bool {
        self.is_mac()
    }

    /// 中文说明: 判断操作系统名称是否匹配 OS/2。
    /// 对齐 Java 方法: `SystemUtil.isOS2`
    #[must_use]
    pub fn is_os2(&self) -> bool {
        self.normalized_name().contains("os/2") || self.normalized_name() == "os2"
    }

    /// 中文说明: 判断操作系统名称是否匹配 Solaris。
    /// 对齐 Java 方法: `SystemUtil.isSolaris`
    #[must_use]
    pub fn is_solaris(&self) -> bool {
        self.normalized_name().contains("solaris")
    }

    /// 中文说明: 判断操作系统名称是否匹配 SunOS。
    /// 对齐 Java 方法: `SystemUtil.isSunOs`
    #[must_use]
    pub fn is_sun_os(&self) -> bool {
        self.normalized_name().contains("sunos")
    }

    /// 中文说明: 判断是否为 Windows 系列操作系统。
    /// 对齐 Java 方法: `SystemUtil.isWindows`
    #[must_use]
    pub fn is_windows(&self) -> bool {
        self.normalized_name().contains("windows")
    }

    fn windows_version(&self, expected: &str) -> bool {
        self.is_windows() && self.version.to_ascii_lowercase().contains(expected)
    }

    /// 中文说明: 判断是否为 Windows 2000。
    /// 对齐 Java 方法: `SystemUtil.isWindows2000`
    #[must_use]
    pub fn is_windows_2000(&self) -> bool {
        self.windows_version("2000")
    }

    /// 中文说明: 判断是否为 Windows 95。
    /// 对齐 Java 方法: `SystemUtil.isWindows95`
    #[must_use]
    pub fn is_windows_95(&self) -> bool {
        self.windows_version("95")
    }

    /// 中文说明: 判断是否为 Windows 98。
    /// 对齐 Java 方法: `SystemUtil.isWindows98`
    #[must_use]
    pub fn is_windows_98(&self) -> bool {
        self.windows_version("98")
    }

    /// 中文说明: 判断是否为 Windows ME。
    /// 对齐 Java 方法: `SystemUtil.isWindowsME`
    #[must_use]
    pub fn is_windows_me(&self) -> bool {
        self.windows_version("me")
    }

    /// 中文说明: 判断是否为 Windows NT。
    /// 对齐 Java 方法: `SystemUtil.isWindowsNT`
    #[must_use]
    pub fn is_windows_nt(&self) -> bool {
        self.windows_version("nt")
    }

    /// 中文说明: 判断是否为 Windows XP。
    /// 对齐 Java 方法: `SystemUtil.isWindowsXP`
    #[must_use]
    pub fn is_windows_xp(&self) -> bool {
        self.windows_version("xp")
    }

    /// 中文说明: 判断是否为 Windows 7。
    /// 对齐 Java 方法: `SystemUtil.isWindows7`
    #[must_use]
    pub fn is_windows_7(&self) -> bool {
        self.windows_version("7")
    }

    /// 中文说明: 判断是否为 Windows 8，不含 8.1。
    /// 对齐 Java 方法: `SystemUtil.isWindows8`
    #[must_use]
    pub fn is_windows_8(&self) -> bool {
        self.windows_version("8") && !self.windows_version("8.1")
    }

    /// 中文说明: 判断是否为 Windows 8.1。
    /// 对齐 Java 方法: `SystemUtil.isWindows8_1`
    #[must_use]
    pub fn is_windows_8_1(&self) -> bool {
        self.windows_version("8.1")
    }

    /// 中文说明: 判断是否为 Windows 10。
    /// 对齐 Java 方法: `SystemUtil.isWindows10`
    #[must_use]
    pub fn is_windows_10(&self) -> bool {
        self.windows_version("10")
    }

    /// 中文说明: 判断是否为 Windows 11。
    /// 对齐 Java 方法: `SystemUtil.isWindows11`
    #[must_use]
    pub fn is_windows_11(&self) -> bool {
        self.windows_version("11")
    }
}

impl fmt::Display for OsInfo {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} {} ({})", self.name, self.version, self.arch)
    }
}

fn system_name_or_fallback() -> String {
    System::name().unwrap_or_else(|| env::consts::OS.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    type OsPredicate = fn(&OsInfo) -> bool;

    #[cfg(windows)]
    const LINE_SEPARATOR_TEST: &str = "\r\n";
    #[cfg(not(windows))]
    const LINE_SEPARATOR_TEST: &str = "\n";
    #[cfg(windows)]
    const PATH_SEPARATOR_TEST: char = ';';
    #[cfg(not(windows))]
    const PATH_SEPARATOR_TEST: char = ':';

    #[test]
    fn predicates_cover_every_hutool_family_and_version() {
        let cases: &[(&str, &str, OsPredicate)] = &[
            ("AIX", "", OsInfo::is_aix),
            ("HP-UX", "", OsInfo::is_hp_ux),
            ("IRIX", "", OsInfo::is_irix),
            ("Linux", "", OsInfo::is_linux),
            ("Darwin", "", OsInfo::is_mac),
            ("Mac OS X", "", OsInfo::is_mac_os_x),
            ("OS/2", "", OsInfo::is_os2),
            ("Solaris", "", OsInfo::is_solaris),
            ("SunOS", "", OsInfo::is_sun_os),
            ("Windows", "2000", OsInfo::is_windows_2000),
            ("Windows", "95", OsInfo::is_windows_95),
            ("Windows", "98", OsInfo::is_windows_98),
            ("Windows", "ME", OsInfo::is_windows_me),
            ("Windows", "NT", OsInfo::is_windows_nt),
            ("Windows", "XP", OsInfo::is_windows_xp),
            ("Windows", "7", OsInfo::is_windows_7),
            ("Windows", "8", OsInfo::is_windows_8),
            ("Windows", "8.1", OsInfo::is_windows_8_1),
            ("Windows", "10", OsInfo::is_windows_10),
            ("Windows", "11", OsInfo::is_windows_11),
        ];
        for (name, version, predicate) in cases {
            let os = OsInfo::from_parts("test-arch", *name, *version);
            assert!(predicate(&os), "{name} {version}");
            assert!(!predicate(&OsInfo::from_parts("x", "unknown", "")));
        }
        let windows_81 = OsInfo::from_parts("x", "Windows", "8.1");
        assert!(windows_81.is_windows());
        assert!(!windows_81.is_windows_8());
    }

    #[test]
    fn collect_populates_separators() {
        let current = OsInfo::collect();
        assert!(!current.arch.is_empty());
        assert!(!current.name.is_empty());
        assert_eq!(current.file_separator, std::path::MAIN_SEPARATOR);
        assert_eq!(current.path_separator, PATH_SEPARATOR_TEST);
        assert_eq!(current.line_separator, LINE_SEPARATOR_TEST);
        assert!(!format!("{current}").is_empty());
    }
}
