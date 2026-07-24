//! 结果集处理器 —— 对齐 Hutool `cn.hutool.db.handler.*`。

use crate::entity::Entity;
use crate::page_result::PageResult;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::BTreeSet;

/// 对齐 Hutool `PageResultHandler`。
#[derive(Debug, Clone)]
pub struct PageResultHandler {
    page: u32,
    page_size: u32,
    total: u64,
}

impl PageResultHandler {
    /// 对齐 Java: `create(PageResult)` 的本地等价。
    #[must_use]
    pub fn create(page: u32, page_size: u32, total: u64) -> Self {
        Self {
            page,
            page_size,
            total,
        }
    }

    /// 对齐 Java 构造。
    #[must_use]
    pub fn new(page: u32, page_size: u32, total: u64) -> Self {
        Self::create(page, page_size, total)
    }

    /// 对齐 Java: `handle`。
    #[must_use]
    pub fn handle(&self, rows: Vec<Entity>) -> PageResult {
        PageResult::new(self.page, self.page_size, self.total, rows)
    }
}
