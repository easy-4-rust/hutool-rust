//! OSHI-aligned hardware and operating-system snapshots backed by `sysinfo`.

use std::{thread, time::Duration};

use sysinfo::{Components, Disks, Networks, Pid, System};

use super::cpu_info::CpuInfo;
use super::disk_info::DiskInfo;
use super::memory_info::MemoryInfo;
use super::network_info::NetworkInfo;
use super::sensor_info::SensorInfo;

/// Aggregate hardware snapshot.
#[derive(Debug, Clone, PartialEq)]
pub struct HardwareInfo {
    /// CPU information.
    pub cpu: CpuInfo,
    /// Memory information.
    pub memory: MemoryInfo,
    /// Disk stores.
    pub disks: Vec<DiskInfo>,
    /// Network interfaces.
    pub networks: Vec<NetworkInfo>,
    /// Hardware sensors.
    pub sensors: Vec<SensorInfo>,
}
