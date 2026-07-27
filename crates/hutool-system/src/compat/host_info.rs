//! 对齐: `cn.hutool.system.HostInfo`
//! 来源: `hutool-system/src/main/java/cn/hutool/system/HostInfo.java`
//! 中文说明: 采集主机名和主机地址，对应 Hutool 的主机信息对象。

use sysinfo::System;

/// 对齐: `cn.hutool.system.HostInfo`
/// 中文说明: 主机身份信息快照，包含主机名和主地址。
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HostInfo {
    /// 中文说明: 主机名。
    pub name: Option<String>,
    /// 中文说明: 主地址；当前 Rust 兼容层暂未主动探测 IP。
    pub address: Option<String>,
}

impl HostInfo {
    /// 中文说明: 采集便携式的主机身份信息。
    /// 对齐 Java 方法: `HostInfo` 构造/初始化逻辑
    #[must_use]
    pub fn collect() -> Self {
        Self {
            name: System::host_name(),
            address: None,
        }
    }
}
