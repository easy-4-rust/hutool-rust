//! 对齐: `cn.hutool.system.JavaSpecInfo`
//! 来源: `hutool-system/src/main/java/cn/hutool/system/JavaSpecInfo.java`
//! 中文说明: 保存 Java 规范层属性快照，兼容 Hutool 的规范名称、版本与供应商信息对象。

use std::env;

/// 对齐: `cn.hutool.system.JavaSpecInfo`
/// 中文说明: Java 规范属性快照，对应 Java `java.specification.*` 系统属性集合。
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct JavaSpecInfo {
    /// 中文说明: 规范名称，对应 Java `java.specification.name`。
    pub name: Option<String>,
    /// 中文说明: 规范版本，对应 Java `java.specification.version`。
    pub version: Option<String>,
    /// 中文说明: 规范供应商，对应 Java `java.specification.vendor`。
    pub vendor: Option<String>,
}

impl JavaSpecInfo {
    /// 中文说明: 检测 Java 规范层环境属性，无需执行 Java。
    /// 对齐 Java 方法: `JavaSpecInfo` 的属性采集逻辑
    #[must_use]
    pub fn detect() -> Self {
        Self {
            name: env::var("JAVA_SPECIFICATION_NAME").ok(),
            version: env::var("JAVA_SPECIFICATION_VERSION").ok(),
            vendor: env::var("JAVA_SPECIFICATION_VENDOR").ok(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_returns_optional_fields() {
        let info = JavaSpecInfo::detect();
        // 字段均为 Option<String>，仅验证结构与 Debug 输出非空
        let _ = info.name;
        let _ = info.version;
        let _ = info.vendor;
        assert!(format!("{info:?}").contains("JavaSpecInfo"));
    }
}
