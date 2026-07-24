//! 结果集处理器 —— 对齐 Hutool `cn.hutool.db.handler.*`。

use crate::entity::Entity;
use crate::page_result::PageResult;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::BTreeSet;

/// 对齐 Hutool `StringHandler`。
#[derive(Debug, Clone, Copy, Default)]
pub struct StringHandler;

impl StringHandler {
    /// 对齐 Java: `create()`。
    #[must_use]
    pub fn create() -> Self {
        Self
    }

    /// 对齐 Java: `handle`。
    #[must_use]
    pub fn handle(&self, rows: &[Entity]) -> Option<String> {
        rows.first()
            .and_then(|e| e.iter().next())
            .map(|(_, v)| match v {
                Value::String(s) => s.clone(),
                Value::Null => String::new(),
                other => other.to_string().trim_matches('"').to_string(),
            })
    }
}
