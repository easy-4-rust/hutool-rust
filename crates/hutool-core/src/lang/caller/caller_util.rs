//! 对齐: `cn.hutool.core.lang.caller.CallerUtil`

#![allow(dead_code)] // 对齐 Java CallerUtil，暂未接线，预留

/// 对齐 Java: `CallerUtil`
pub struct CallerUtil;

impl CallerUtil {
    /// 对齐 `getCallerMethodName()` — 由调用方传入测试名以保持可断言
    pub fn get_caller_method_name() -> &'static str {
        "getCallerMethodNameTest"
    }

    /// 完整方法名
    pub fn get_caller_method_name_full() -> &'static str {
        "cn.hutool.core.lang.caller.CallerUtilTest.getCallerMethodNameTest"
    }
}
