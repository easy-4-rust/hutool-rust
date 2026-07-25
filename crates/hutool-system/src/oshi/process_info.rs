//! 对齐: `cn.hutool.system.oshi.OSProcess`
//! 来源: hutool-system/src/main/java/cn/hutool/system/oshi/OSProcess.java
//! 中文说明: 进程快照，包含进程 ID、名称、常驻内存、虚拟内存和运行时间

use std::{thread, time::Duration};

use sysinfo::{Components, Disks, Networks, Pid, System};

use super::oshi_util::OshiUtil;

/// 对齐: `cn.hutool.system.oshi.OSProcess`
/// 中文说明: 进程快照，由 [`OshiUtil::current_process`] 返回，包含进程 ID、名称、内存和运行时间
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessInfo {
    /// 中文说明: 进程标识符
    pub pid: u32,
    /// 中文说明: 可执行文件/进程名称
    pub name: String,
    /// 中文说明: 常驻内存（字节）
    pub memory: u64,
    /// 中文说明: 虚拟内存（字节）
    pub virtual_memory: u64,
    /// 中文说明: 进程运行时间（秒）
    pub run_time: u64,
}
