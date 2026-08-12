//! 动态实体 —— 对齐 Hutool `cn.hutool.db.ActiveEntity`。

use crate::db::{Db, DbResult};
use crate::entity::Entity;

/// 对齐 Hutool `ActiveEntity`。
#[derive(Debug, Clone)]
pub struct ActiveEntity {
    entity: Entity,
    db: Db,
}

impl ActiveEntity {
    /// 对齐 Java: `ActiveEntity(Db db, String tableName)`.
    #[must_use]
    pub fn new(db: Db, table_name: impl Into<String>) -> Self {
        Self {
            entity: Entity::create_table(table_name),
            db,
        }
    }

    /// 对齐 Java: `setFieldNames`.
    pub fn set_field_names(
        &mut self,
        fields: impl IntoIterator<Item = impl Into<String>>,
    ) -> &mut Self {
        self.entity.set_field_names(fields);
        self
    }

    /// 对齐 Java: `load()`.
    pub async fn load(&mut self) -> DbResult<&mut Self> {
        let rows = self
            .db
            .find_fields(self.entity.field_names(), &self.entity)
            .await?;
        if let Some(first) = rows.into_iter().next() {
            self.entity = first;
        }
        Ok(self)
    }

    /// 对齐 Java: `getTableName()`.
    #[must_use]
    pub fn table_name(&self) -> Option<&str> {
        self.entity.table_name()
    }

    /// 对齐 Java: `isEmpty()`.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entity.is_empty()
    }
}
