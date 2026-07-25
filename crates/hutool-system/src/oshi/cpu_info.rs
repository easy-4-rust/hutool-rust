//! 对齐: `cn.hutool.system.oshi.CpuInfo` (CentralProcessor)
//! 来源: hutool-system/src/main/java/cn/hutool/system/oshi/CpuInfo.java
//! 中文说明: 便携式的 CPU 利用率视图，包含逻辑 CPU 数量、总使用率、系统/用户/等待/空闲百分比和处理器型号

use std::{thread, time::Duration};

use sysinfo::{Components, Disks, Networks, Pid, System};

use super::cpu_ticks::CpuTicks;

/// 对齐: `cn.hutool.system.oshi.CpuInfo`
/// 中文说明: 便携式的 CPU 利用率视图，对应 Hutool 的 `CpuInfo`
#[derive(Debug, Clone, PartialEq)]
pub struct CpuInfo {
    /// 中文说明: 逻辑 CPU 数量
    pub cpu_num: usize,
    /// 中文说明: 总繁忙百分比
    pub total: f32,
    /// 中文说明: 系统百分比（可用时）
    pub system: f32,
    /// 中文说明: 用户百分比（可用时）
    pub user: f32,
    /// 中文说明: I/O 等待百分比（可用时）
    pub wait: f32,
    /// 中文说明: 空闲百分比
    pub free: f32,
    /// 中文说明: 处理器品牌/型号
    pub cpu_model: String,
    /// 中文说明: 原始 tick 快照（可用时）
    pub ticks: CpuTicks,
}

impl CpuInfo {
    /// 中文说明: 创建标准化的 CPU 快照
    /// 对齐 Java 方法: `CpuInfo` 构造函数
    #[must_use]
    pub fn new(cpu_num: usize, used: f32, cpu_model: impl Into<String>, ticks: CpuTicks) -> Self {
        let total = used.clamp(0.0, 100.0);
        Self {
            cpu_num,
            total,
            system: 0.0,
            user: total,
            wait: 0.0,
            free: 100.0 - total,
            cpu_model: cpu_model.into(),
            ticks,
        }
    }

    /// 中文说明: 返回总使用率百分比
    /// 对齐 Java 方法: `CpuInfo.getUsed`
    #[must_use]
    pub const fn used(&self) -> f32 {
        self.total
    }
}
