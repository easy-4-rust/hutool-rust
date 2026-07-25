//! 对齐: `cn.hutool.system.oshi` (OshiUtil 硬件信息采集模块)
//! 来源: hutool-system/src/main/java/cn/hutool/system/oshi/*.java
//! 中文说明: 提供与 OSHI 对齐的硬件和操作系统快照，基于 `sysinfo` 库实现，涵盖 CPU、内存、磁盘、网络、传感器和进程信息

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
