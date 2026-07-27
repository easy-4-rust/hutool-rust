//! 对齐: `cn.hutool.system.ManagementInfo`
//! 来源: `hutool-system/src/main/java/cn/hutool/system/ManagementInfo.java`
//! 中文说明: 替代 `java.lang.management.ManagementFactory` 的便携式管理信息聚合。

use crate::{compilation_info::CompilationInfo, os_info::OsInfo, oshi::MemoryInfo, oshi::ProcessInfo};

/// 对齐: `java.lang.management.ManagementFactory` (替代)
/// 中文说明: 便携式的原生管理信息集合，替代 JVM 的 ManagementFactory
#[derive(Debug, Clone, PartialEq)]
pub struct ManagementInfo {
    /// 中文说明: 当前进程信息。
    pub process: Option<ProcessInfo>,
    /// 中文说明: 主机内存信息。
    pub memory: MemoryInfo,
    /// 中文说明: 操作系统属性。
    pub os: OsInfo,
    /// 中文说明: 编译属性。
    pub compilation: CompilationInfo,
    /// 中文说明: 可用并行度，作为便携式的线程容量度量。
    pub thread_capacity: usize,
}

impl ManagementInfo {
    /// 中文说明: 创建显式的 ManagementInfo，便于测试。
    /// 对齐 Java 方法: `ManagementInfo` 构造函数
    #[must_use]
    pub fn from_parts(
        process: Option<ProcessInfo>,
        memory: MemoryInfo,
        os: OsInfo,
        compilation: CompilationInfo,
        thread_capacity: usize,
    ) -> Self {
        Self {
            process,
            memory,
            os,
            compilation,
            thread_capacity,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::oshi::MemoryInfo;

    #[test]
    fn from_parts_round_trips() {
        let memory = MemoryInfo {
            total: 100,
            used: 50,
            available: 80,
            swap_total: 0,
            swap_used: 0,
        };
        let os = OsInfo::from_parts("x86_64", "Linux", "5.0");
        let compilation = CompilationInfo::from_parts("rustc", "x86_64", false);
        let info = ManagementInfo::from_parts(None, memory, os, compilation, 4);
        assert_eq!(info.memory.total, 100);
        assert_eq!(info.os.name, "Linux");
        assert_eq!(info.compilation.compiler, "rustc");
        assert_eq!(info.thread_capacity, 4);
        assert!(info.process.is_none());
    }
}