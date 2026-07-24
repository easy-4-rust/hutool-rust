//! OSHI-aligned hardware and operating-system snapshots backed by `sysinfo`.

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

/// Mature `sysinfo`-backed counterpart of Hutool's `OshiUtil`.
#[derive(Debug, Clone, Copy, Default)]
pub struct OshiUtil;

impl OshiUtil {
    /// Returns a fully refreshed `sysinfo` system.
    #[must_use]
    pub fn system() -> System {
        System::new_all()
    }

    /// Returns the current process when it remains visible during collection.
    #[must_use]
    pub fn current_process() -> Option<ProcessInfo> {
        let system = Self::system();
        let pid = Pid::from_u32(std::process::id());
        process_info(&system, pid)
    }

    /// Looks up a process in an existing system snapshot.
    #[must_use]
    pub fn process(system: &System, pid: u32) -> Option<ProcessInfo> {
        process_info(system, Pid::from_u32(pid))
    }

    /// Returns current physical and swap memory counters.
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

    /// Samples aggregate CPU utilization after `interval`.
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

    /// Collects disk-store summaries.
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

    /// Collects network-interface counters.
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

    /// Collects temperature sensors supported by the host.
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

    /// Collects the complete hardware view.
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
