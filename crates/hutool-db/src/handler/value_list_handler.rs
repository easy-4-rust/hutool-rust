//! 结果集处理器 —— 对齐 Hutool `cn.hutool.db.handler.*`。

use crate::entity::Entity;
use crate::page_result::PageResult;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::BTreeSet;

use super::handle_helper::HandleHelper;

/// 对齐 Hutool `ValueListHandler`。
#[derive(Debug, Clone, Copy, Default)]
pub struct ValueListHandler;

impl ValueListHandler {
    /// 对齐 Java: `create()`。
    #[must_use]
    pub fn create() -> Self {
        Self
    }

    /// 对齐 Java: `handle` —— 每行按列序取值。
    #[must_use]
    pub fn handle(&self, rows: &[Entity]) -> Vec<Vec<Value>> {
        rows.iter().map(HandleHelper::handle_row_to_list).collect()
    }
}
