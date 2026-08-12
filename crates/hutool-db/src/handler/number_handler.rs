//! 结果集处理器 —— 对齐 Hutool `cn.hutool.db.handler.*`。

use crate::entity::Entity;
use serde_json::Value;

/// 对齐 Hutool `NumberHandler`。
#[derive(Debug, Clone, Copy, Default)]
pub struct NumberHandler;

impl NumberHandler {
    /// 对齐 Java: `create()`。
    #[must_use]
    pub fn create() -> Self {
        Self
    }

    /// 对齐 Java: `handle` —— 取首行首列数值。
    #[must_use]
    pub fn handle(&self, rows: &[Entity]) -> Option<i64> {
        rows.first()
            .and_then(|e| e.iter().next())
            .and_then(|(_, v)| match v {
                Value::Number(n) => n.as_i64().or_else(|| n.as_f64().map(|f| f as i64)),
                Value::String(s) => s.parse().ok(),
                Value::Bool(b) => Some(i64::from(*b)),
                _ => None,
            })
    }
}
