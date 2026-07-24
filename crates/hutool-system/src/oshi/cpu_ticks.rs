//! OSHI-aligned hardware and operating-system snapshots backed by `sysinfo`.

use std::{thread, time::Duration};

use sysinfo::{Components, Disks, Networks, Pid, System};

/// CPU tick counters. Platforms that do not expose a counter leave it at zero.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CpuTicks {
    /// Idle ticks.
    pub idle: u64,
    /// Nice-priority ticks.
    pub nice: u64,
    /// Hardware interrupt ticks.
    pub irq: u64,
    /// Software interrupt ticks.
    pub soft_irq: u64,
    /// Hypervisor steal ticks.
    pub steal: u64,
    /// System ticks.
    pub system: u64,
    /// User ticks.
    pub user: u64,
    /// I/O wait ticks.
    pub io_wait: u64,
}

impl CpuTicks {
    /// Creates a complete tick snapshot.
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

    /// Returns the saturating total of every counter.
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
