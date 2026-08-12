//! 对齐: `cn.hutool.core.lang.func.LambdaUtil`

#![allow(dead_code)] // 对齐 Java LambdaUtil，暂未接线，预留

/// 对齐 Java: `LambdaUtil` — Rust 无方法引用，提供可断言的名称解析辅助
pub struct LambdaUtil;

impl LambdaUtil {
    /// 对齐 `getMethodName` — 传入已解析名
    pub fn get_method_name(name: &str) -> &str {
        name
    }

    /// 对齐 `getFieldName` — 去掉 get/is 前缀并小写首字母
    pub fn get_field_name(method_name: &str) -> String {
        let rest = method_name
            .strip_prefix("get")
            .or_else(|| method_name.strip_prefix("is"))
            .unwrap_or(method_name);
        let mut chars = rest.chars();
        match chars.next() {
            None => String::new(),
            Some(c) => c.to_lowercase().collect::<String>() + chars.as_str(),
        }
    }

    /// 对齐 `resolve` 引用种类标签
    pub fn resolve_kind(kind: &str) -> &str {
        kind
    }
}
