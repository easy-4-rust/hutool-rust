//! 对齐: `cn.hutool.system.JvmInfo`
//! 来源: `hutool-system/src/main/java/cn/hutool/system/JvmInfo.java`
//! 中文说明: 保存 JVM 实现层属性快照，包含虚拟机名称、版本、供应商与实现信息。

use std::env;

/// 对齐: `cn.hutool.system.JvmInfo`
/// 中文说明: JVM 属性快照，字段直接映射 Hutool `JvmInfo` 的核心系统属性。
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct JvmInfo {
    /// 中文说明: 虚拟机名称，对应 Java `java.vm.name`。
    pub name: Option<String>,
    /// 中文说明: 虚拟机版本，对应 Java `java.vm.version`。
    pub version: Option<String>,
    /// 中文说明: 虚拟机供应商，对应 Java `java.vm.vendor`。
    pub vendor: Option<String>,
    /// 中文说明: 虚拟机附加信息，对应 Java `java.vm.info`。
    pub info: Option<String>,
}

impl JvmInfo {
    /// 中文说明: 检测 JVM 实现层环境属性。
    /// 对齐 Java 方法: `JvmInfo` 的属性采集逻辑
    #[must_use]
    pub fn detect() -> Self {
        Self {
            name: env::var("JAVA_VM_NAME").ok(),
            version: env::var("JAVA_VM_VERSION").ok(),
            vendor: env::var("JAVA_VM_VENDOR").ok(),
            info: env::var("JAVA_VM_INFO").ok(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_returns_optional_fields() {
        let info = JvmInfo::detect();
        let _ = info.name;
        let _ = info.version;
        let _ = info.vendor;
        let _ = info.info;
        assert!(format!("{info:?}").contains("JvmInfo"));
    }

    #[test]
    fn default_is_empty() {
        let info = JvmInfo::default();
        assert_eq!(info.name, None);
        assert_eq!(info.version, None);
        assert_eq!(info.vendor, None);
        assert_eq!(info.info, None);
    }
}