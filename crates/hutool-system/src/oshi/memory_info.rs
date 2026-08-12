//! 对齐: `cn.hutool.system.oshi.GlobalMemory`
//! 来源: hutool-system/src/main/java/cn/hutool/system/oshi/GlobalMemory.java
//! 中文说明: 物理内存快照，包含总内存、已用内存、可用内存和交换空间信息

/// 对齐: `cn.hutool.system.oshi.GlobalMemory`
/// 中文说明: 物理内存快照，包含总内存、已用内存、可用内存和交换空间信息
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoryInfo {
    /// 中文说明: 总内存（字节）
    pub total: u64,
    /// 中文说明: 已用内存（字节）
    pub used: u64,
    /// 中文说明: 可用内存（字节）
    pub available: u64,
    /// 中文说明: 交换空间总量（字节）
    pub swap_total: u64,
    /// 中文说明: 已用交换空间（字节）
    pub swap_used: u64,
}
