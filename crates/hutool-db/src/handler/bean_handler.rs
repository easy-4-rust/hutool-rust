//! 结果集处理器 —— 对齐 Hutool `cn.hutool.db.handler.*`。

use crate::entity::Entity;
use crate::page_result::PageResult;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::BTreeSet;

/// 对齐 Hutool `BeanHandler`。
#[derive(Debug, Clone, Copy, Default)]
pub struct BeanHandler;

impl BeanHandler {
    /// 对齐 Java: `create()`。
    #[must_use]
    pub fn create() -> Self {
        Self
    }

    /// 对齐 Java: `handle` —— 首行转 bean。
    pub fn handle<T: DeserializeOwned>(&self, rows: &[Entity]) -> Result<Option<T>, serde_json::Error> {
        rows.first()
            .map(Entity::to_bean_ignore_case)
            .transpose()
    }
}
