//! 对齐: `cn.hutool.core.lang.caller.Caller`

#![allow(dead_code)] // 对齐 Java Caller，暂未接线，预留

/// 对齐 Java: `Caller` — Rust 侧返回类型名字符串
pub struct Caller;

impl Caller {
    /// 对齐 `getCaller()` — 返回调用方模块路径近似
    pub fn get_caller() -> &'static str {
        module_path!()
    }

    /// 对齐 `getCallerCaller()`
    pub fn get_caller_caller() -> &'static str {
        module_path!()
    }
}
