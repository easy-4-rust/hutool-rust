//! 对齐: `cn.hutool.core.thread.RejectPolicy`
//! 来源: hutool-core/src/main/java/cn/hutool/core/thread/RejectPolicy.java

/// 对齐 Java enum: `cn.hutool.core.thread.RejectPolicy`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)] // 对齐 Java 枚举常量名（DISCARD_OLDEST 等）
pub enum RejectPolicy {
    /// 对齐 Java 枚举常量: `ABORT`
    ABORT,
    /// 对齐 Java 枚举常量: `DISCARD`
    DISCARD,
    /// 对齐 Java 枚举常量: `DISCARD_OLDEST`
    DISCARD_OLDEST,
    /// 对齐 Java 枚举常量: `CALLER_RUNS`
    CALLER_RUNS,
    /// 对齐 Java 枚举常量: `BLOCK`
    BLOCK,
}

impl RejectPolicy {
    /// 对齐 `getValue()` — 返回自身作为 Handler 标识。
    pub fn get_value(self) -> Self {
        self
    }
}
