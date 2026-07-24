//! OSHI-aligned hardware and operating-system snapshots backed by `sysinfo`.

use std::{thread, time::Duration};

use sysinfo::{Components, Disks, Networks, Pid, System};

/// Hardware sensor snapshot.
#[derive(Debug, Clone, PartialEq)]
pub struct SensorInfo {
    /// Sensor label.
    pub label: String,
    /// Temperature in Celsius, if reported.
    pub temperature: Option<f32>,
}
