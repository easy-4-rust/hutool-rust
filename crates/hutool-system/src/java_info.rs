//! 对齐: `cn.hutool.system.JavaInfo`
//! 来源: `hutool-system/src/main/java/cn/hutool/system/JavaInfo.java`
//! 中文说明: 保存 Java 实现层属性快照，兼容 Hutool 的 Java 版本、供应商与供应商网址视图。

use std::env;

use crate::system_props::SystemPropsKeys;

/// 对齐: `cn.hutool.system.JavaInfo`
/// 中文说明: 解析后的 Java 实现属性；Rust 侧通过环境变量模拟 Java 系统属性来源。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct JavaInfo {
    /// 中文说明: Java 版本，对应 Java `java.version`。
    pub version: Option<String>,
    /// 中文说明: Java 供应商，对应 Java `java.vendor`。
    pub vendor: Option<String>,
    /// 中文说明: Java 供应商网址，对应 Java `java.vendor.url`。
    pub vendor_url: Option<String>,
}

impl JavaInfo {
    /// 中文说明: 创建显式的 Java 属性对象。
    /// 对齐 Java 方法: `JavaInfo` 构造函数
    #[must_use]
    pub fn new(
        version: Option<String>,
        vendor: Option<String>,
        vendor_url: Option<String>,
    ) -> Self {
        Self {
            version,
            vendor,
            vendor_url,
        }
    }

    /// 中文说明: 检测可选的 Java 环境属性，无需启动 JVM。
    /// 对齐 Java 方法: `JavaInfo` 的静态检测逻辑
    #[must_use]
    pub fn detect() -> Self {
        Self::new(
            env::var(SystemPropsKeys::JAVA_VERSION).ok(),
            env::var("JAVA_VENDOR").ok(),
            env::var("JAVA_VENDOR_URL").ok(),
        )
    }

    /// 中文说明: 返回主要版本组件的十进制表示（如 `1.8`）。
    /// 对齐 Java 方法: `JavaInfo.getVersionFloat`
    #[must_use]
    pub fn version_float(&self) -> Option<f32> {
        let (major, minor) = self.version_components()?;
        format!("{major}.{minor}").parse().ok()
    }

    fn version_components(&self) -> Option<(u32, u32)> {
        let mut parts = self
            .version
            .as_deref()?
            .trim_start_matches(|character: char| !character.is_ascii_digit())
            .split(|character: char| !character.is_ascii_digit())
            .filter(|part| !part.is_empty());
        let major = parts.next()?.parse::<u32>().ok()?;
        let minor = parts
            .next()
            .and_then(|part| part.parse::<u32>().ok())
            .unwrap_or(0);
        Some((major, minor))
    }

    /// 中文说明: 返回 Java 特性版本号（`1.8` 变为 `8`）。
    /// 对齐 Java 方法: `JavaInfo.getVersionInt`
    #[must_use]
    pub fn version_int(&self) -> Option<u32> {
        let (major, minor) = self.version_components()?;
        if major == 1 { Some(minor) } else { Some(major) }
    }

    /// 中文说明: 检查是否为指定的特性版本。
    /// 对齐 Java 方法: `JavaInfo.isVersion`
    #[must_use]
    pub fn is_version(&self, version: u32) -> bool {
        self.version_int() == Some(version)
    }

    /// 中文说明: 检查是否至少为指定的特性版本。
    /// 对齐 Java 方法: `JavaInfo.isVersionAtLeast`
    #[must_use]
    pub fn is_version_at_least(&self, version: u32) -> bool {
        self.version_int().is_some_and(|current| current >= version)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn versions_and_helpers_are_deterministic() {
        let java8 = JavaInfo::new(Some("1.8.0_412".into()), None, None);
        assert_eq!(java8.version_float(), Some(1.8));
        assert_eq!(java8.version_int(), Some(8));
        assert!(java8.is_version(8));
        assert!(java8.is_version_at_least(7));
        assert!(!java8.is_version_at_least(9));

        let java17 = JavaInfo::new(
            Some("openjdk-17.0.10".into()),
            Some("vendor".into()),
            Some("https://example.invalid".into()),
        );
        assert_eq!(java17.version_int(), Some(17));
        assert!(java17.is_version(17));
        assert_eq!(
            JavaInfo::new(Some("bad".into()), None, None).version_float(),
            None
        );
        assert_eq!(JavaInfo::default().version_int(), None);
        assert!(!JavaInfo::default().is_version(0));
        assert!(format!("{java17:?}").contains("vendor"));
    }

    #[test]
    fn saturating_version_int_clamps_huge_inputs() {
        assert_eq!(
            JavaInfo::new(Some("17.999999999999999999999".into()), None, None).version_int(),
            Some(17)
        );
        assert_eq!(
            JavaInfo::new(Some("999999999999999999999".into()), None, None).version_int(),
            None
        );
    }
}
