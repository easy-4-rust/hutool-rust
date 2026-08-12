//! 对齐: `cn.hutool.system.CompilationInfo`
//! 来源: `hutool-system/src/main/java/cn/hutool/system/CompilationInfo.java`
//! 中文说明: 替代 `java.lang.management.RuntimeMXBean` 的便携式编译信息。

use std::env;

/// 对齐: `java.lang.management.RuntimeMXBean` (替代)
/// 中文说明: 原生 Rust 编译器/运行时信息，替代 JVM 专有的 MXBeans
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompilationInfo {
    /// 中文说明: 编译器类型。
    pub compiler: &'static str,
    /// 中文说明: 目标架构。
    pub target_arch: &'static str,
    /// 中文说明: 是否启用调试断言。
    pub debug_assertions: bool,
}

impl CompilationInfo {
    /// 中文说明: 从当前进程环境采集编译信息。
    /// 对齐 Java 方法: `SystemUtil.getCompilationInfo`
    #[must_use]
    pub fn detect() -> Self {
        Self {
            compiler: "rustc",
            target_arch: env::consts::ARCH,
            debug_assertions: cfg!(debug_assertions),
        }
    }

    /// 中文说明: 使用显式参数创建 `CompilationInfo`，便于测试。
    /// 对齐 Java 方法: `CompilationInfo` 构造函数
    #[must_use]
    pub const fn from_parts(
        compiler: &'static str,
        target_arch: &'static str,
        debug_assertions: bool,
    ) -> Self {
        Self {
            compiler,
            target_arch,
            debug_assertions,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_uses_rustc_target() {
        let info = CompilationInfo::detect();
        assert_eq!(info.compiler, "rustc");
        assert!(!info.target_arch.is_empty());
        assert_eq!(info.debug_assertions, cfg!(debug_assertions));
    }

    #[test]
    fn from_parts_round_trips() {
        let info = CompilationInfo::from_parts("rustc", "x86_64", true);
        assert_eq!(info.compiler, "rustc");
        assert_eq!(info.target_arch, "x86_64");
        assert!(info.debug_assertions);
    }
}
