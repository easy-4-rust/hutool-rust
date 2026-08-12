//! 表达式引擎抽象与默认实现，对齐 hutool 的 `cn.hutool.extra.expression.*`。
//!
//! - `ExpressionEngine` trait：对齐 Java `ExpressionEngine`（`eval` 三参语义）
//! - `RhaiEngine`：基于 [`rhai`](https://crates.io/crates/rhai) 的默认引擎，
//!   覆盖 hutool `ExpressionUtil` 的变量绑定 + 表达式求值语义
//!   （对齐 Java 默认引擎 aviator 的算术/逻辑/字符串拼接）
//! - 其他引擎（Aviator/JEXL/SpEL）为 Java 特有，Rust 侧通过 trait 注入扩展

use std::collections::HashMap;

use crate::HutoolException;

/// 表达式引擎抽象，对齐 `cn.hutool.extra.expression.ExpressionEngine`。
pub trait ExpressionEngine: Send + Sync {
    /// 在给定上下文（变量绑定）下执行表达式字符串。
    ///
    /// 对齐 Java `eval(String, Map<String, Object>, Collection<Class<?>>)`：
    /// - `context`：变量名 → `值（serde_json::Value` 承载）
    /// - `allow_class_set`：类型白名单（Rust 无反射概念，作为元信息透传，引擎可忽略）
    fn eval(
        &self,
        expression: &str,
        context: &HashMap<String, serde_json::Value>,
        allow_class_set: &[&str],
    ) -> Result<serde_json::Value, HutoolException>;

    /// 获取底层原始引擎（Java 返回 `Object`）
    fn raw_engine(&self) -> Option<&dyn std::any::Any>;
}

/// rhai 引擎封装，对齐 hutool 默认 aviator 引擎的求值语义。
///
/// Java hutool 的 `ExpressionFactory` 通过 SPI 加载首个可用引擎（默认 aviator）；
/// Rust 侧 `RhaiEngine` 作为内置默认实现，避免运行期 SPI 发现。
#[derive(Default)]
pub struct RhaiEngine {
    engine: rhai::Engine,
}

impl RhaiEngine {
    /// 创建带默认配置的引擎实例。
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// 将上下文变量压入 rhai `Scope`，按 JSON 类型映射：
    /// - `Number` → `i64`/`f64`（rhai 数值类型）
    /// - `String` → `String`、`Bool` → `bool`
    /// - 其他（Array/Object/Null）→ `rhai::Dynamic`
    fn build_scope(context: &HashMap<String, serde_json::Value>) -> rhai::Scope<'static> {
        let mut scope = rhai::Scope::new();
        for (name, value) in context {
            match value {
                serde_json::Value::Null => {
                    scope.push_dynamic(name.clone(), rhai::Dynamic::UNIT);
                }
                serde_json::Value::Bool(b) => {
                    scope.push(name.clone(), *b);
                }
                serde_json::Value::Number(n) => {
                    if let Some(i) = n.as_i64() {
                        scope.push(name.clone(), i);
                    } else if let Some(f) = n.as_f64() {
                        scope.push(name.clone(), f);
                    }
                }
                serde_json::Value::String(s) => {
                    scope.push(name.clone(), s.clone());
                }
                serde_json::Value::Array(_) | serde_json::Value::Object(_) => {
                    // 复杂结构序列化为 JSON 字符串（对齐 Java 的 Map/List 透传）
                    scope.push(name.clone(), value.to_string());
                }
            }
        }
        scope
    }

    /// 将 rhai 求值结果转回 `serde_json::Value`。
    fn dynamic_to_value(value: rhai::Dynamic) -> serde_json::Value {
        if value.is_unit() {
            return serde_json::Value::Null;
        }
        if let Ok(b) = value.as_bool() {
            return serde_json::Value::Bool(b);
        }
        if let Ok(i) = value.as_int() {
            return serde_json::Value::Number(serde_json::Number::from(i));
        }
        if let Ok(f) = value.as_float()
            && let Some(n) = serde_json::Number::from_f64(f) {
                return serde_json::Value::Number(n);
            }
        if value.is_string() {
            return serde_json::Value::String(value.into_string().unwrap_or_default());
        }
        // 兜底：不可识别类型序列化为字符串（保持 Java 的 Object 透明语义）
        serde_json::Value::String(format!("{value}"))
    }
}

impl ExpressionEngine for RhaiEngine {
    fn eval(
        &self,
        expression: &str,
        context: &HashMap<String, serde_json::Value>,
        _allow_class_set: &[&str],
    ) -> Result<serde_json::Value, HutoolException> {
        let mut scope = Self::build_scope(context);
        self.engine
            .eval_expression_with_scope::<rhai::Dynamic>(&mut scope, expression)
            .map(Self::dynamic_to_value)
            .map_err(|error| HutoolException::FromCause {
                message: format!("rhai eval failed: {error}"),
                source: Box::new(error),
            })
    }

    fn raw_engine(&self) -> Option<&dyn std::any::Any> {
        Some(&self.engine)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_arithmetic_with_bindings() {
        let engine = RhaiEngine::new();
        let mut ctx = HashMap::new();
        ctx.insert("a".to_string(), serde_json::json!(40));
        ctx.insert("b".to_string(), serde_json::json!(2));
        let result = engine.eval("a + b * 1", &ctx, &[]).expect("arithmetic");
        assert_eq!(result, serde_json::json!(42));
    }

    #[test]
    fn eval_string_concat() {
        let engine = RhaiEngine::new();
        let mut ctx = HashMap::new();
        ctx.insert("name".to_string(), serde_json::json!("hutool"));
        let result = engine
            .eval("\"hello, \" + name", &ctx, &[])
            .expect("concat");
        assert_eq!(result, serde_json::json!("hello, hutool"));
    }

    #[test]
    fn eval_boolean_logic() {
        let engine = RhaiEngine::new();
        let mut ctx = HashMap::new();
        ctx.insert("flag".to_string(), serde_json::json!(true));
        ctx.insert("count".to_string(), serde_json::json!(3));
        let result = engine.eval("flag && count > 2", &ctx, &[]).expect("logic");
        assert_eq!(result, serde_json::json!(true));
    }

    #[test]
    fn eval_syntax_error_propagates() {
        let engine = RhaiEngine::new();
        let result = engine.eval("a +", &HashMap::new(), &[]);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            HutoolException::FromCause { .. }
        ));
    }

    #[test]
    fn eval_integer_division_truncates() {
        let engine = RhaiEngine::new();
        let mut ctx = HashMap::new();
        ctx.insert("a".to_string(), serde_json::json!(10));
        // rhai 整数除法截断，需显式浮点
        let result = engine.eval("a / 4", &ctx, &[]).expect("int div");
        assert_eq!(result, serde_json::json!(2));
    }

    #[test]
    fn eval_float_division() {
        let engine = RhaiEngine::new();
        let mut ctx = HashMap::new();
        ctx.insert("a".to_string(), serde_json::json!(10.0));
        let result = engine.eval("a / 4.0", &ctx, &[]).expect("float");
        assert_eq!(result, serde_json::json!(2.5));
    }

    #[test]
    fn raw_engine_returns_rhai_engine() {
        let engine = RhaiEngine::new();
        let raw = engine.raw_engine();
        assert!(
            raw.and_then(|any| any.downcast_ref::<rhai::Engine>())
                .is_some()
        );
    }
}
