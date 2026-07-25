//! 对齐: `cn.hutool.system.oshi.CpuInfo` (tick 计数器部分)
//! 来源: hutool-system/src/main/java/cn/hutool/system/oshi/CpuInfo.java
//! 中文说明: CPU tick 计数器，不暴露计数器的平台保留为零

use std::{thread, time::Duration};

use sysinfo::{Components, Disks, Networks, Pid, System};

/// 对齐: `cn.hutool.system.oshi.CpuInfo` (tick 部分)
/// 中文说明: CPU tick 计数器，不暴露计数器的平台保留为零
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CpuTicks {
    /// 中文说明: 空闲 tick
    pub idle: u64,
    /// 中文说明: Nice 优先级 tick
    pub nice: u64,
    /// 中文说明: 硬件中断 tick
    pub irq: u64,
    /// 中文说明: 软件中断 tick
    pub soft_irq: u64,
    /// 中文说明: 虚拟化窃取 tick
    pub steal: u64,
    /// 中文说明: 系统 tick
    pub system: u64,
    /// 中文说明: 用户 tick
    pub user: u64,
    /// 中文说明: I/O 等待 tick
    pub io_wait: u64,
}

impl CpuTicks {
    /// 中文说明: 创建完整的 tick 快照
    /// 对齐 Java 方法: `CpuTicks` 构造函数
    #[must_use]
    pub const fn new(values: [u64; 8]) -> Self {
        Self {
            idle: values[0],
            nice: values[1],
            irq: values[2],
            soft_irq: values[3],
            steal: values[4],
            system: values[5],
            user: values[6],
            io_wait: values[7],
        }
    }

    /// 中文说明: 返回所有计数器的饱和总和
    /// 对齐 Java 方法: `CpuTicks.getTotalCpu`
    #[must_use]
    pub fn total_cpu(self) -> u64 {
        [
            self.idle,
            self.nice,
            self.irq,
            self.soft_irq,
            self.steal,
            self.system,
            self.user,
            self.io_wait,
        ]
        .into_iter()
        .fold(0, u64::saturating_add)
    }
}
