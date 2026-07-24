//! OSHI-aligned hardware and operating-system snapshots backed by `sysinfo`.

use std::{thread, time::Duration};

use sysinfo::{Components, Disks, Networks, Pid, System};

/// Network-interface transfer snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkInfo {
    /// Interface name.
    pub name: String,
    /// Total received bytes since boot/counter reset.
    pub received: u64,
    /// Total transmitted bytes since boot/counter reset.
    pub transmitted: u64,
}
