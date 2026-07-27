//! 对齐: `cn.hutool.system` (system-util 与 system-snapshot 整合模块)
//! 来源: hutool-system/src/main/java/cn/hutool/system/SystemUtil.java
//! 中文说明: 提供跨平台的主机信息、操作系统信息、运行时内存信息等系统属性查询工具，对应 Hutool `system` 模块入口

#![forbid(unsafe_code)]

use sysinfo::System;

mod compilation_info;
mod host_info;
mod java_info;
mod java_runtime_info;
mod java_spec_info;
mod jvm_info;
mod jvm_spec_info;
mod management_info;
mod os_info;
pub mod prelude;
mod runtime_info;
mod system_props;
mod system_util;
mod user_info;

pub mod oshi;

pub use compilation_info::CompilationInfo;
pub use host_info::HostInfo;
pub use java_info::JavaInfo;
pub use java_runtime_info::JavaRuntimeInfo;
pub use java_spec_info::JavaSpecInfo;
pub use jvm_info::JvmInfo;
pub use jvm_spec_info::JvmSpecInfo;
pub use management_info::ManagementInfo;
pub use os_info::OsInfo;
pub use runtime_info::RuntimeInfo;
pub use system_props::SystemPropsKeys;
pub use system_util::SystemUtil;
pub use user_info::UserInfo;

pub use oshi::{
    CpuInfo, CpuTicks, DiskInfo, HardwareInfo, MemoryInfo, NetworkInfo, OshiUtil, ProcessInfo,
    SensorInfo,
};

/// 对齐: `cn.hutool.system.SystemUtil` (系统快照)
/// 中文说明: 当前主机的某一时刻系统快照，包含主机名、操作系统、CPU、内存和运行时间等信息
#[derive(Debug, Clone, PartialEq)]
pub struct SystemSnapshot {
    /// 中文说明: 操作系统报告的主机名
    pub host_name: Option<String>,
    /// 中文说明: 操作系统名称
    pub os_name: Option<String>,
    /// 中文说明: 操作系统版本
    pub os_version: Option<String>,
    /// 中文说明: 逻辑 CPU 数量
    pub cpu_count: usize,
    /// 中文说明: 所有逻辑 CPU 的平均使用率百分比
    pub cpu_usage_percent: f32,
    /// 中文说明: 总物理内存（字节）
    pub total_memory: u64,
    /// 中文说明: 已使用内存（字节）
    pub used_memory: u64,
    /// 中文说明: 系统运行时间（秒）
    pub uptime_seconds: u64,
}

impl SystemSnapshot {
    /// 中文说明: 采集一份全新的系统快照
    /// 对齐 Java 方法: `SystemUtil` 的系统属性收集逻辑
    #[must_use]
    pub fn collect() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        let cpus = system.cpus();
        Self {
            host_name: System::host_name(),
            os_name: System::name(),
            os_version: System::os_version(),
            cpu_count: cpus.len(),
            cpu_usage_percent: system.global_cpu_usage(),
            total_memory: system.total_memory(),
            used_memory: system.used_memory(),
            uptime_seconds: System::uptime(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot_has_sane_resource_values() {
        let snapshot = SystemSnapshot::collect();
        assert!(snapshot.cpu_count > 0);
        assert!(snapshot.total_memory > 0);
        assert!(snapshot.used_memory <= snapshot.total_memory);
        assert!(snapshot.cpu_usage_percent >= 0.0);
    }
}