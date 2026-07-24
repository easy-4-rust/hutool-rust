//! 表达式引擎 facade，对齐 hutool 的 `cn.hutool.extra.expression.*`。
//!
//! 提供 ExpressionEngine trait + ExpressionUtil 静态门面 + 6 个构造器签名。
//! 各具体引擎（Aviator/JEXL/JfireEL/Mvel/QLExpress/Rhino/SpEL）需要外部 Java crate，
//! 属于 unsafe-to-copy，暂不在 hutool-extra 中实现。

use std::collections::HashMap;

use crate::HutoolException;

/// 表达式引擎抽象，对齐 `cn.hutool.extra.expression.ExpressionEngine`。
pub trait ExpressionEngine: Send + Sync {
    /// 在给定上下文（变量绑定）下执行表达式字符串。
    fn eval(
        &self,
        expression: &str,
        context: &HashMap<String, serde_json::Value>,
        allow_class_set: &[&str],
    ) -> Result<serde_json::Value, HutoolException>;

    /// 获取底层原始引擎（Java 返回 `Object`）
    fn raw_engine(&self) -> Option<&dyn std::any::Any>;
}
