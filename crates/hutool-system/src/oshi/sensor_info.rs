//! 对齐: `cn.hutool.system.oshi.Sensors`
//! 来源: hutool-system/src/main/java/cn/hutool/system/oshi/Sensors.java
//! 中文说明: 硬件传感器快照，包含传感器标签和温度信息

use std::{thread, time::Duration};

use sysinfo::{Components, Disks, Networks, Pid, System};

/// 对齐: `cn.hutool.system.oshi.Sensors`
/// 中文说明: 硬件传感器快照，包含传感器标签和温度信息
#[derive(Debug, Clone, PartialEq)]
pub struct SensorInfo {
    /// 中文说明: 传感器标签
    pub label: String,
    /// 中文说明: 摄氏温度（如报告）
    pub temperature: Option<f32>,
}
