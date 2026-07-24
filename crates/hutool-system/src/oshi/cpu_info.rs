//! OSHI-aligned hardware and operating-system snapshots backed by `sysinfo`.

use std::{thread, time::Duration};

use sysinfo::{Components, Disks, Networks, Pid, System};

use super::cpu_ticks::CpuTicks;

/// Portable CPU utilization view corresponding to Hutool's `CpuInfo`.
#[derive(Debug, Clone, PartialEq)]
pub struct CpuInfo {
    /// Logical CPU count.
    pub cpu_num: usize,
    /// Total busy percentage.
    pub total: f32,
    /// System percentage when available.
    pub system: f32,
    /// User percentage when available.
    pub user: f32,
    /// I/O wait percentage when available.
    pub wait: f32,
    /// Idle percentage.
    pub free: f32,
    /// Processor brand/model.
    pub cpu_model: String,
    /// Raw tick snapshot when available.
    pub ticks: CpuTicks,
}

impl CpuInfo {
    /// Creates a normalized CPU snapshot.
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

    /// Returns total used percentage.
    #[must_use]
    pub const fn used(&self) -> f32 {
        self.total
    }
}
