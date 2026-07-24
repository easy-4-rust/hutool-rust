//! 结果集处理器 —— 对齐 Hutool `cn.hutool.db.handler.*`。

use crate::entity::Entity;
use crate::page_result::PageResult;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::BTreeSet;

/// 对齐 Hutool `HandleHelper`：行/列 → Entity 转换辅助。
pub struct HandleHelper;

impl HandleHelper {
    /// 对齐 Java: `handleRow` —— 由列名与值构造 Entity。
    #[must_use]
    pub fn handle_row(columns: impl IntoIterator<Item = (impl Into<String>, Value)>) -> Entity {
        let mut entity = Entity::create();
        for (k, v) in columns {
            entity.set_value(k, v);
        }
        entity
    }

    /// 对齐 Java: `handleRowToList` —— 单行按列序输出值列表。
    #[must_use]
    pub fn handle_row_to_list(entity: &Entity) -> Vec<Value> {
        entity.iter().map(|(_, v)| v.clone()).collect()
    }

    /// 对齐 Java: `handleRs` —— 多行 Entity 列表（已是 Entity 时恒等）。
    #[must_use]
    pub fn handle_rs(rows: Vec<Entity>) -> Vec<Entity> {
        rows
    }

    /// 对齐 Java: `handleRsToBeanList`。
    pub fn handle_rs_to_bean_list<T: DeserializeOwned>(
        rows: &[Entity],
    ) -> Result<Vec<T>, serde_json::Error> {
        rows.iter().map(Entity::to_bean_ignore_case).collect()
    }
}
