//! 结果集处理器 —— 对齐 Hutool `cn.hutool.db.handler.*`。

use crate::entity::Entity;
use crate::page_result::PageResult;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::BTreeSet;

use super::handle_helper::HandleHelper;

/// 对齐 Hutool `BeanListHandler`。
#[derive(Debug, Clone, Copy, Default)]
pub struct BeanListHandler;

impl BeanListHandler {
    /// 对齐 Java: `create()`。
    #[must_use]
    pub fn create() -> Self {
        Self
    }

    /// 对齐 Java: `handle`。
    pub fn handle<T: DeserializeOwned>(&self, rows: &[Entity]) -> Result<Vec<T>, serde_json::Error> {
        HandleHelper::handle_rs_to_bean_list(rows)
    }
}
