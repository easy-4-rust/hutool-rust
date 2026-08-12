//! 对齐: `cn.hutool.system.oshi.HardwareInfo`
//! 来源: hutool-system/src/main/java/cn/hutool/system/oshi/HardwareInfo.java
//! 中文说明: 聚合硬件快照，包含 CPU、内存、磁盘、网络和传感器信息

use super::cpu_info::CpuInfo;
use super::disk_info::DiskInfo;
use super::memory_info::MemoryInfo;
use super::network_info::NetworkInfo;
use super::sensor_info::SensorInfo;

/// 对齐: `cn.hutool.system.oshi.HardwareInfo`
/// 中文说明: 聚合硬件快照，包含 CPU、内存、磁盘、网络和传感器信息
#[derive(Debug, Clone, PartialEq)]
pub struct HardwareInfo {
    /// 中文说明: CPU 信息
    pub cpu: CpuInfo,
    /// 中文说明: 内存信息
    pub memory: MemoryInfo,
    /// 中文说明: 磁盘存储列表
    pub disks: Vec<DiskInfo>,
    /// 中文说明: 网络接口列表
    pub networks: Vec<NetworkInfo>,
    /// 中文说明: 硬件传感器列表
    pub sensors: Vec<SensorInfo>,
}
