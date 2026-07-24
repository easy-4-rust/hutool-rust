//! 结果集处理器 —— 对齐 Hutool `cn.hutool.db.handler.*`。

use crate::entity::Entity;
use crate::page_result::PageResult;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::BTreeSet;

/// 对齐 Hutool `EntitySetHandler`。
#[derive(Debug, Clone, Copy, Default)]
pub struct EntitySetHandler {
    case_insensitive: bool,
}

impl EntitySetHandler {
    /// 对齐 Java: `create()`。
    #[must_use]
    pub fn create() -> Self {
        Self::default()
    }

    /// 对齐 Java 构造。
    #[must_use]
    pub fn new(case_insensitive: bool) -> Self {
        Self { case_insensitive }
    }

    /// 对齐 Java: `handle` —— 按字段签名去重。
    #[must_use]
    pub fn handle(&self, rows: &[Entity]) -> Vec<Entity> {
        let _ = self.case_insensitive;
        let mut seen = BTreeSet::new();
        let mut out = Vec::new();
        for row in rows {
            let key = row
                .iter()
                .map(|(k, v)| format!("{k}={v}"))
                .collect::<Vec<_>>()
                .join("&");
            if seen.insert(key) {
                out.push(row.clone());
            }
        }
        out
    }
}
