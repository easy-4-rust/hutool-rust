//! 表达式引擎 facade，对齐 hutool 的 `cn.hutool.extra.expression.*`。
//!
//! 提供 ExpressionEngine trait + ExpressionUtil 静态门面 + 6 个构造器签名。
//! 各具体引擎（Aviator/JEXL/JfireEL/Mvel/QLExpress/Rhino/SpEL）需要外部 Java crate，
//! 属于 unsafe-to-copy，暂不在 hutool-extra 中实现。

use std::collections::HashMap;

use crate::HutoolException;

use super::expression_engine::ExpressionEngine;

/// 表达式工具类，对齐 `cn.hutool.extra.expression.ExpressionUtil`。
pub struct ExpressionUtil;

impl ExpressionUtil {
    /// 对齐 `ExpressionUtil.eval(String expression, Map<String, Object> context)`
    pub fn eval(
        expression: &str,
        context: &HashMap<String, serde_json::Value>,
    ) -> Result<serde_json::Value, HutoolException> {
        Self::eval_with_classes(expression, context, &[])
    }

    /// 对齐 `ExpressionUtil.eval(String, Map, Collection<Class<?>>)`
    pub fn eval_with_classes(
        _expression: &str,
        _context: &HashMap<String, serde_json::Value>,
        _allow_class_set: &[&str],
    ) -> Result<serde_json::Value, HutoolException> {
        // Phase 1.4: 等具体 engine（spel-engine 等）实现后填充
        Err(HutoolException::Message(
            "ExpressionUtil::eval requires a concrete ExpressionEngine implementation".into(),
        ))
    }

    /// 对齐 `ExpressionUtil.getEngine()`：返回默认引擎实例
    pub fn get_engine() -> Result<Box<dyn ExpressionEngine>, HutoolException> {
        Err(HutoolException::Message(
            "ExpressionUtil::get_engine requires a concrete ExpressionEngine implementation".into(),
        ))
    }
}
