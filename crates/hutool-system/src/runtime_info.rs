//! 对齐: `cn.hutool.system.RuntimeInfo`
//! 来源: `hutool-system/src/main/java/cn/hutool/system/RuntimeInfo.java`
//! 中文说明: 采集 Rust 进程/运行时内存信息，对应 Hutool 的 `RuntimeInfo`，包含最大内存、总内存、可用内存和进程内存。

use crate::oshi::OshiUtil;

/// 对齐: `cn.hutool.system.RuntimeInfo`
/// 中文说明: Rust 进程/运行时内存信息，对应 Hutool 的 `RuntimeInfo`，包含最大内存、总内存、可用内存和进程内存
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuntimeInfo {
    /// 中文说明: 最大可用内存（原生 Rust 为物理内存总量）。
    pub max_memory: u64,
    /// 中文说明: 总物理内存。
    pub total_memory: u64,
    /// 中文说明: 可用物理内存。
    pub free_memory: u64,
    /// 中文说明: 当前进程常驻内存。
    pub process_memory: u64,
}

impl RuntimeInfo {
    /// 中文说明: 采集运行时内存计数器
    /// 对齐 Java 方法: `RuntimeInfo` 构造/初始化逻辑
    #[must_use]
    pub fn collect() -> Self {
        let memory = OshiUtil::memory();
        let process_memory = OshiUtil::current_process().map_or(0, |process| process.memory);
        Self {
            max_memory: memory.total,
            total_memory: memory.total,
            free_memory: memory.available,
            process_memory,
        }
    }

    /// 中文说明: 从显式输入构造 RuntimeInfo，便于测试。
    /// 对齐 Java 方法: `RuntimeInfo` 构造函数
    #[must_use]
    pub const fn from_parts(
        max_memory: u64,
        total_memory: u64,
        free_memory: u64,
        process_memory: u64,
    ) -> Self {
        Self {
            max_memory,
            total_memory,
            free_memory,
            process_memory,
        }
    }

    /// 中文说明: 返回不超过原生主机限制的可用内存。
    /// 对齐 Java 方法: `RuntimeInfo.getUsableMemory`
    #[must_use]
    pub fn usable_memory(self) -> u64 {
        self.free_memory.saturating_add(self.process_memory)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collect_returns_positive_memory() {
        let runtime = RuntimeInfo::collect();
        assert!(runtime.total_memory > 0);
        assert!(runtime.usable_memory() >= runtime.free_memory);
    }

    #[test]
    fn usable_memory_saturates() {
        let runtime = RuntimeInfo::from_parts(0, 0, u64::MAX, u64::MAX);
        assert_eq!(runtime.usable_memory(), u64::MAX);
    }

    #[test]
    fn from_parts_round_trips() {
        let runtime = RuntimeInfo::from_parts(100, 80, 40, 20);
        assert_eq!(runtime.max_memory, 100);
        assert_eq!(runtime.total_memory, 80);
        assert_eq!(runtime.free_memory, 40);
        assert_eq!(runtime.process_memory, 20);
        assert_eq!(runtime.usable_memory(), 60);
    }
}