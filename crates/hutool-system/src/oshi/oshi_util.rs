//! 对齐: `cn.hutool.system.oshi.OshiUtil`
//! 来源: hutool-system/src/main/java/cn/hutool/system/oshi/OshiUtil.java
//! 中文说明: 硬件信息采集工具类，提供系统、进程、内存、CPU、磁盘、网络、传感器和硬件信息的便捷查询方法

use std::{thread, time::Duration};

use sysinfo::{Components, Disks, Networks, Pid, System};

use super::cpu_info::CpuInfo;
use super::cpu_ticks::CpuTicks;
use super::disk_info::DiskInfo;
use super::hardware_info::HardwareInfo;
use super::memory_info::MemoryInfo;
use super::network_info::NetworkInfo;
use super::process_info::ProcessInfo;
use super::sensor_info::SensorInfo;

/// 对齐: `cn.hutool.system.oshi.OshiUtil`
/// 中文说明: 成熟的 `sysinfo` 支持的硬件信息采集工具，对应 Hutool 的 `OshiUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct OshiUtil;

impl OshiUtil {
    /// 中文说明: 返回一个完全刷新的 `sysinfo` 系统实例
    /// 对齐 Java 方法: `OshiUtil.getSystem`
    #[must_use]
    pub fn system() -> System {
        System::new_all()
    }

    /// 中文说明: 返回当前进程信息（在采集期间可见时）
    /// 对齐 Java 方法: `OshiUtil.getCurrentProcess`
    #[must_use]
    pub fn current_process() -> Option<ProcessInfo> {
        let system = Self::system();
        let pid = Pid::from_u32(std::process::id());
        process_info(&system, pid)
    }

    /// 中文说明: 在现有系统快照中查找指定进程
    /// 对齐 Java 方法: `OshiUtil.getProcess`
    #[must_use]
    pub fn process(system: &System, pid: u32) -> Option<ProcessInfo> {
        process_info(system, Pid::from_u32(pid))
    }

    /// 中文说明: 返回当前物理内存和交换内存计数器
    /// 对齐 Java 方法: `OshiUtil.getMemory`
    #[must_use]
    pub fn memory() -> MemoryInfo {
        let system = Self::system();
        MemoryInfo {
            total: system.total_memory(),
            used: system.used_memory(),
            available: system.available_memory(),
            swap_total: system.total_swap(),
            swap_used: system.used_swap(),
        }
    }

    /// 中文说明: 在指定时间间隔后采样聚合 CPU 利用率
    /// 对齐 Java 方法: `OshiUtil.getCpuInfo`
    #[must_use]
    pub fn cpu_info(interval: Duration) -> CpuInfo {
        let mut system = System::new_all();
        if !interval.is_zero() {
            thread::sleep(interval);
            system.refresh_cpu_usage();
        }
        let cpus = system.cpus();
        let model = cpus
            .first()
            .map_or_else(String::new, |cpu| cpu.brand().to_owned());
        CpuInfo::new(
            cpus.len(),
            system.global_cpu_usage(),
            model,
            CpuTicks::default(),
        )
    }

    /// 中文说明: 采集磁盘存储摘要信息
    /// 对齐 Java 方法: `OshiUtil.getDiskStores`
    #[must_use]
    pub fn disk_stores() -> Vec<DiskInfo> {
        Disks::new_with_refreshed_list()
            .list()
            .iter()
            .map(|disk| DiskInfo {
                name: disk.name().to_string_lossy().into_owned(),
                mount_point: disk.mount_point().to_string_lossy().into_owned(),
                total_space: disk.total_space(),
                available_space: disk.available_space(),
            })
            .collect()
    }

    /// 中文说明: 采集网络接口计数器
    /// 对齐 Java 方法: `OshiUtil.getNetworkInterfaces`
    #[must_use]
    pub fn network_interfaces() -> Vec<NetworkInfo> {
        Networks::new_with_refreshed_list()
            .iter()
            .map(|(name, data)| NetworkInfo {
                name: name.clone(),
                received: data.total_received(),
                transmitted: data.total_transmitted(),
            })
            .collect()
    }

    /// 中文说明: 采集主机支持的温度传感器
    /// 对齐 Java 方法: `OshiUtil.getSensors`
    #[must_use]
    pub fn sensors() -> Vec<SensorInfo> {
        Components::new_with_refreshed_list()
            .iter()
            .map(|component| SensorInfo {
                label: component.label().to_owned(),
                temperature: component.temperature(),
            })
            .collect()
    }

    /// 中文说明: 采集完整的硬件视图
    /// 对齐 Java 方法: `OshiUtil.getHardware`
    #[must_use]
    pub fn hardware() -> HardwareInfo {
        HardwareInfo {
            cpu: Self::cpu_info(Duration::ZERO),
            memory: Self::memory(),
            disks: Self::disk_stores(),
            networks: Self::network_interfaces(),
            sensors: Self::sensors(),
        }
    }
}

fn process_info(system: &System, pid: Pid) -> Option<ProcessInfo> {
    let process = system.process(pid)?;
    Some(ProcessInfo {
        pid: pid.as_u32(),
        name: process.name().to_string_lossy().into_owned(),
        memory: process.memory(),
        virtual_memory: process.virtual_memory(),
        run_time: process.run_time(),
    })
}
