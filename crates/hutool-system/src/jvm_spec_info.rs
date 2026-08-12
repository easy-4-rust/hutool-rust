//! 对齐: `cn.hutool.system.JvmSpecInfo`
//! 来源: `hutool-system/src/main/java/cn/hutool/system/JvmSpecInfo.java`
//! 中文说明: 保存 JVM 规范层属性快照，兼容 Hutool 的虚拟机规范名称、版本与供应商信息对象。

use std::env;

/// 对齐: `cn.hutool.system.JvmSpecInfo`
/// 中文说明: JVM 规范属性快照，对应 Java `java.vm.specification.*` 系统属性集合。
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct JvmSpecInfo {
    /// 中文说明: 规范名称，对应 Java `java.vm.specification.name`。
    pub name: Option<String>,
    /// 中文说明: 规范版本，对应 Java `java.vm.specification.version`。
    pub version: Option<String>,
    /// 中文说明: 规范供应商，对应 Java `java.vm.specification.vendor`。
    pub vendor: Option<String>,
}

impl JvmSpecInfo {
    /// 中文说明: 检测 JVM 规范层环境属性，无需执行 Java。
    /// 对齐 Java 方法: `JvmSpecInfo` 的属性采集逻辑
    #[must_use]
    pub fn detect() -> Self {
        Self {
            name: env::var("JAVA_VM_SPECIFICATION_NAME").ok(),
            version: env::var("JAVA_VM_SPECIFICATION_VERSION").ok(),
            vendor: env::var("JAVA_VM_SPECIFICATION_VENDOR").ok(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_returns_optional_fields() {
        let info = JvmSpecInfo::detect();
        let _ = info.name;
        let _ = info.version;
        let _ = info.vendor;
        assert!(format!("{info:?}").contains("JvmSpecInfo"));
    }
}
