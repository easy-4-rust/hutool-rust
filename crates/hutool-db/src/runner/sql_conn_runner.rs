//! Db 运行器门面 —— 对齐 Hutool `AbstractDb` / `SqlConnRunner` / `DialectRunner` / `SqlExecutor`。
//!
//! 均委托现有 `Db`（SQLx pool），避免 JDBC Connection / Statement 全局状态。

use crate::db::{Db, DbResult};
use crate::dialect::impls::Dialect as DialectTrait;
use crate::dialect::name::DialectName;
use crate::entity::Entity;
use crate::hutool_page::HutoolPage;
use crate::page_result::PageResult;
use crate::sql::condition::{Condition, LikeType};
use crate::wrapper::Wrapper;
use serde_json::Value;
use sqlx::SqlitePool;
use std::collections::HashMap;

use super::abstract_db::AbstractDb;

/// 对齐 Hutool `SqlConnRunner`：连接级 CRUD 委托 `Db`。
#[derive(Debug, Clone)]
pub struct SqlConnRunner {
    inner: AbstractDb,
}

impl SqlConnRunner {
    /// 对齐 Java: `SqlConnRunner.create(Connection)` —— 使用注入 pool。
    #[must_use]
    pub fn create(pool: SqlitePool) -> Self {
        Self {
            inner: AbstractDb::new(pool),
        }
    }

    /// 从 `Db` 创建。
    #[must_use]
    pub fn from_db(db: Db) -> Self {
        Self {
            inner: AbstractDb::from_db(db),
        }
    }

    /// 访问内部门面。
    #[must_use]
    pub fn abstract_db(&self) -> &AbstractDb {
        &self.inner
    }

    /// 对齐 Java: `insert`。
    pub async fn insert(&self, entity: &Entity) -> DbResult<u64> {
        self.inner.insert(entity).await
    }

    /// 对齐 Java: `find`。
    pub async fn find(&self, where_entity: &Entity) -> DbResult<Vec<Entity>> {
        self.inner.find(where_entity).await
    }

    /// 对齐 Java: `findAll`。
    pub async fn find_all(&self, table: &str) -> DbResult<Vec<Entity>> {
        self.inner.find_all(table).await
    }

    /// 对齐 Java: `findBy`。
    pub async fn find_by(&self, table: &str, conditions: &[Condition]) -> DbResult<Vec<Entity>> {
        self.inner.find_by(table, conditions).await
    }

    /// 对齐 Java: `findLike`。
    pub async fn find_like(
        &self,
        table: &str,
        field: &str,
        value: &str,
        like_type: LikeType,
    ) -> DbResult<Vec<Entity>> {
        self.inner.find_like(table, field, value, like_type).await
    }

    /// 对齐 Java: `count`。
    pub async fn count(&self, where_entity: &Entity) -> DbResult<u64> {
        self.inner.count(where_entity).await
    }

    /// 对齐 Java: `page`。
    pub async fn page(
        &self,
        sql: &str,
        page: &HutoolPage,
        params: &[Value],
    ) -> DbResult<PageResult> {
        self.inner.page(sql, page, params).await
    }
}
