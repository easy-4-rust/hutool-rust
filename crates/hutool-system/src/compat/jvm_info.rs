//! 对齐: `cn.hutool.system.JvmInfo`
//! 来源: `hutool-system/src/main/java/cn/hutool/system/JvmInfo.java`
//! 中文说明: 保存 JVM 实现层属性快照，包含虚拟机名称、版本、供应商与实现信息。

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
