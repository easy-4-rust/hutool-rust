//! 结果集处理器 —— 对齐 Hutool `cn.hutool.db.handler.*`。

use crate::entity::Entity;
use crate::page_result::PageResult;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::BTreeSet;

use super::rs_handler::RsHandler;

/// 对齐 Hutool `EntityHandler`。
#[derive(Debug, Clone, Copy, Default)]
pub struct EntityHandler {
    case_insensitive: bool,
}

impl EntityHandler {
    /// 对齐 Java: `create()`。
    #[must_use]
    pub fn create() -> Self {
        Self::default()
    }

    /// 对齐 Java: `EntityHandler(boolean caseInsensitive)`。
    #[must_use]
    pub fn new(case_insensitive: bool) -> Self {
        Self { case_insensitive }
    }

    /// 对齐 Java: `handle` —— 取首行。
    #[must_use]
    pub fn handle(&self, rows: &[Entity]) -> Option<Entity> {
        let _ = self.case_insensitive;
        rows.first().cloned()
    }
}

impl RsHandler<Option<Entity>> for EntityHandler {
    fn handle(&self, rows: &[Entity]) -> Option<Entity> {
        self.handle(rows)
    }
}
