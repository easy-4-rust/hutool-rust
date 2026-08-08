//! 表达式引擎工具类，对齐 hutool 的 `cn.hutool.extra.expression.ExpressionUtil`。
//!
//! 静态门面通过默认引擎（`RhaiEngine`）求值；可通过 `set_engine` 注入自定义引擎。

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::HutoolException;

use super::expression_engine::{ExpressionEngine, RhaiEngine};

/// 表达式工具类，对齐 `cn.hutool.extra.expression.ExpressionUtil`。
pub struct ExpressionUtil;

static DEFAULT_ENGINE: OnceLock<Box<dyn ExpressionEngine>> = OnceLock::new();

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
        expression: &str,
        context: &HashMap<String, serde_json::Value>,
        allow_class_set: &[&str],
    ) -> Result<serde_json::Value, HutoolException> {
        Self::get_engine().eval(expression, context, allow_class_set)
    }

    /// 对齐 `ExpressionUtil.getEngine()`：返回默认引擎引用。
    ///
    /// Java 通过 `ExpressionFactory.get()` 按 SPI 加载首个引擎（默认 aviator）；
    /// Rust 侧默认返回内置 `RhaiEngine`，可用 `set_engine` 覆盖。
    pub fn get_engine() -> &'static dyn ExpressionEngine {
        DEFAULT_ENGINE
            .get_or_init(|| Box::<RhaiEngine>::default())
            .as_ref()
    }

    /// 注入自定义引擎，覆盖默认 `RhaiEngine`。
    ///
    /// 对齐 Java `ExpressionFactory` 的 SPI 自定义场景；进程级一次性设置，
    /// 后续 `get_engine` 返回注入实现。
    pub fn set_engine(engine: Box<dyn ExpressionEngine>) -> Result<(), HutoolException> {
        DEFAULT_ENGINE
            .set(engine)
            .map_err(|_| HutoolException::Message("engine already initialized".into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_via_static_facade() {
        let mut ctx = HashMap::new();
        ctx.insert("x".to_string(), serde_json::json!(6));
        ctx.insert("y".to_string(), serde_json::json!(7));
        let result = ExpressionUtil::eval("x * y", &ctx).expect("eval");
        assert_eq!(result, serde_json::json!(42));
    }

    #[test]
    fn get_engine_is_singleton() {
        let a = std::ptr::from_ref(ExpressionUtil::get_engine());
        let b = std::ptr::from_ref(ExpressionUtil::get_engine());
        assert_eq!(a, b, "get_engine should return the same singleton");
    }
}
