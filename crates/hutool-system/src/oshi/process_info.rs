//! OSHI-aligned hardware and operating-system snapshots backed by `sysinfo`.

use std::{thread, time::Duration};

use sysinfo::{Components, Disks, Networks, Pid, System};

use super::oshi_util::OshiUtil;

/// Process snapshot returned by [`OshiUtil::current_process`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessInfo {
    /// Process identifier.
    pub pid: u32,
    /// Executable/process name.
    pub name: String,
    /// Resident memory in bytes.
    pub memory: u64,
    /// Virtual memory in bytes.
    pub virtual_memory: u64,
    /// Process runtime in seconds.
    pub run_time: u64,
}
