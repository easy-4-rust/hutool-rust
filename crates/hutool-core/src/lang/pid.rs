//! 对齐: `cn.hutool.core.lang.Pid`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/Pid.java
//!
//! Java 通过 `ManagementFactory.getRuntimeMXBean().getName()` 解析 `pid@host`；
//! Rust 直接使用 `std::process::id()`，语义等价于获取当前进程 PID。

#![allow(dead_code)] // 对齐 Java Pid，暂未接线，预留

use std::sync::OnceLock;

/// 对齐 Java: `cn.hutool.core.lang.Pid`（单例枚举）
#[derive(Debug, Clone, Copy)]
pub struct Pid;

static CACHED: OnceLock<i32> = OnceLock::new();

impl Pid {
    /// 对齐 Java: `Pid.INSTANCE.get()` — 缓存当前进程 ID。
    #[must_use]
    pub fn get() -> i32 {
        *CACHED.get_or_init(|| std::process::id() as i32)
    }
}

#[cfg(test)]
mod pid_idiomatic_parity {
    use super::*;

    /// 对齐 Java Pid.get 可执行证据：返回正整数且同进程内稳定。
    #[test]
    fn pid_get_is_positive_and_stable() {
        let a = Pid::get();
        let b = Pid::get();
        assert!(a > 0, "pid must be positive");
        assert_eq!(a, b);
        assert_eq!(a, std::process::id() as i32);
    }
}
