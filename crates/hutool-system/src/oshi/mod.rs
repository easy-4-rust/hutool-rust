//! OSHI-aligned hardware and operating-system snapshots backed by `sysinfo`.

use std::{thread, time::Duration};

use sysinfo::{Components, Disks, Networks, Pid, System};

mod cpu_ticks;
mod cpu_info;
mod process_info;
mod memory_info;
mod disk_info;
mod network_info;
mod sensor_info;
mod hardware_info;
mod oshi_util;

pub use cpu_ticks::CpuTicks;
pub use cpu_info::CpuInfo;
pub use process_info::ProcessInfo;
pub use memory_info::MemoryInfo;
pub use disk_info::DiskInfo;
pub use network_info::NetworkInfo;
pub use sensor_info::SensorInfo;
pub use hardware_info::HardwareInfo;
pub use oshi_util::OshiUtil;
