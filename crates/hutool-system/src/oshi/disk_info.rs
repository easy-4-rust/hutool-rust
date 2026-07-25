//! 对齐: `cn.hutool.system.oshi.HWDiskStore`
//! 来源: hutool-system/src/main/java/cn/hutool/system/oshi/HWDiskStore.java
//! 中文说明: 磁盘存储快照，包含设备名称、挂载点、总空间和可用空间

use std::{thread, time::Duration};

use sysinfo::{Components, Disks, Networks, Pid, System};

/// 对齐: `cn.hutool.system.oshi.HWDiskStore`
/// 中文说明: 磁盘存储快照，包含设备名称、挂载点、总空间和可用空间
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiskInfo {
    /// 中文说明: 设备名称
    pub name: String,
    /// 中文说明: 挂载点
    pub mount_point: String,
    /// 中文说明: 总空间（字节）
    pub total_space: u64,
    /// 中文说明: 可用空间（字节）
    pub available_space: u64,
}
