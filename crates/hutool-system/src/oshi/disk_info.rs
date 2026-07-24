//! OSHI-aligned hardware and operating-system snapshots backed by `sysinfo`.

use std::{thread, time::Duration};

use sysinfo::{Components, Disks, Networks, Pid, System};

/// Disk-store snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiskInfo {
    /// Device name.
    pub name: String,
    /// Mount point.
    pub mount_point: String,
    /// Total bytes.
    pub total_space: u64,
    /// Available bytes.
    pub available_space: u64,
}
