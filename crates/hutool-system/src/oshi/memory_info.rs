//! OSHI-aligned hardware and operating-system snapshots backed by `sysinfo`.

use std::{thread, time::Duration};

use sysinfo::{Components, Disks, Networks, Pid, System};

/// Physical memory snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoryInfo {
    /// Total memory bytes.
    pub total: u64,
    /// Used memory bytes.
    pub used: u64,
    /// Available memory bytes.
    pub available: u64,
    /// Total swap bytes.
    pub swap_total: u64,
    /// Used swap bytes.
    pub swap_used: u64,
}
