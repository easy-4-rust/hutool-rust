//! 结果集处理器 —— 对齐 Hutool `cn.hutool.db.handler.*`。

use crate::entity::Entity;
use crate::page_result::PageResult;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::BTreeSet;

use super::rs_handler::RsHandler;

/// 对齐 Hutool `EntityListHandler`。
#[derive(Debug, Clone, Copy, Default)]
pub struct EntityListHandler {
    case_insensitive: bool,
}

impl EntityListHandler {
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

    /// 对齐 Java: `handle`。
    #[must_use]
    pub fn handle(&self, rows: &[Entity]) -> Vec<Entity> {
        let _ = self.case_insensitive;
        rows.to_vec()
    }
}

impl RsHandler<Vec<Entity>> for EntityListHandler {
    fn handle(&self, rows: &[Entity]) -> Vec<Entity> {
        self.handle(rows)
    }
}
