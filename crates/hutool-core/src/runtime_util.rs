//! 对齐: `cn.hutool.core.util.RuntimeUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/RuntimeUtil.java
//!
//! Rust 使用 `std::process` 与系统 API 提供进程/内存信息。

use std::process::{Command, Output, Stdio};

use crate::{CoreError, Result};

/// 对齐 Java: `cn.hutool.core.util.RuntimeUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct RuntimeUtil;

impl RuntimeUtil {
    /// 对齐 Java: `RuntimeUtil.execForStr(String...)`
    pub fn exec_for_str(cmds: &[&str]) -> Result<String> {
        let output = Self::exec(cmds)?;
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    /// 对齐 Java: `RuntimeUtil.execForLines(String...)`
    pub fn exec_for_lines(cmds: &[&str]) -> Result<Vec<String>> {
        let text = Self::exec_for_str(cmds)?;
        if text.is_empty() {
            return Ok(Vec::new());
        }
        Ok(text.lines().map(str::to_string).collect())
    }

    /// 对齐 Java: `RuntimeUtil.exec(String...)`
    pub fn exec(cmds: &[&str]) -> Result<Output> {
        if cmds.is_empty() {
            return Err(CoreError::InvalidArgument {
                name: "cmds",
                reason: "command list must not be empty",
            });
        }
        let mut command = Command::new(cmds[0]);
        if cmds.len() > 1 {
            command.args(&cmds[1..]);
        }
        command
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(CoreError::Io)
    }

    /// 对齐 Java: `RuntimeUtil.getResult(Process)`
    pub fn get_result(output: &Output) -> String {
        String::from_utf8_lossy(&output.stdout).to_string()
    }

    /// 对齐 Java: `RuntimeUtil.getErrorResult(Process)`
    pub fn get_error_result(output: &Output) -> String {
        String::from_utf8_lossy(&output.stderr).to_string()
    }

    /// 对齐 Java: `RuntimeUtil.getProcessorCount()`
    pub fn get_processor_count() -> usize {
        std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1)
    }

    /// 对齐 Java: `RuntimeUtil.getUsableMemory()`
    pub fn get_usable_memory() -> u64 {
        Self::sys_memory_info().map(|(_, available)| available).unwrap_or(0)
    }

    /// 对齐 Java: `RuntimeUtil.getTotalMemory()`
    pub fn get_total_memory() -> u64 {
        Self::sys_memory_info()
            .map(|(total, _)| total)
            .unwrap_or(0)
    }

    /// 对齐 Java: `RuntimeUtil.getPid()`
    pub fn get_pid() -> u32 {
        std::process::id()
    }

    /// 读取系统内存信息（总量/可用），单位字节。
    fn sys_memory_info() -> Option<(u64, u64)> {
        #[cfg(target_os = "macos")]
        {
            let mut cmd = Command::new("sysctl");
            cmd.args(["-n", "hw.memsize"]);
            let total = cmd.output().ok().and_then(|out| {
                String::from_utf8(out.stdout)
                    .ok()
                    .and_then(|s| s.trim().parse().ok())
            })?;
            let mut vm = Command::new("vm_stat");
            let out = vm.output().ok()?;
            let text = String::from_utf8_lossy(&out.stdout);
            let page_size = text
                .lines()
                .find_map(|line| line.strip_prefix("Mach Virtual Memory Statistics: (page size of "))
                .and_then(|rest| rest.split_whitespace().next())
                .and_then(|size| size.parse::<u64>().ok())
                .unwrap_or(4096);
            let mut free_pages = 0_u64;
            for line in text.lines() {
                if let Some(value) = line.split(':').nth(1) {
                    let pages = value.trim().trim_end_matches('.').parse::<u64>().ok();
                    if line.starts_with("Pages free:") {
                        free_pages += pages.unwrap_or(0);
                    }
                }
            }
            Some((total, free_pages.saturating_mul(page_size)))
        }
        #[cfg(target_os = "linux")]
        {
            let mut content = String::new();
            std::fs::File::open("/proc/meminfo")
                .ok()?
                .read_to_string(&mut content)
                .ok()?;
            let mut total = 0_u64;
            let mut available = 0_u64;
            for line in content.lines() {
                if let Some(kb) = line.strip_prefix("MemTotal:") {
                    total = kb.trim().trim_end_matches(" kB").parse::<u64>().ok()? * 1024;
                } else if let Some(kb) = line.strip_prefix("MemAvailable:") {
                    available = kb.trim().trim_end_matches(" kB").parse::<u64>().ok()? * 1024;
                }
            }
            Some((total, available))
        }
        #[cfg(not(any(target_os = "macos", target_os = "linux")))]
        {
            None
        }
    }
}
