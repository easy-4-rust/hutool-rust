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

use super::sql_conn_runner::SqlConnRunner;

/// 对齐 Hutool `DialectRunner`：携带方言元数据的 runner。
#[derive(Debug, Clone)]
pub struct DialectRunner {
    inner: SqlConnRunner,
    dialect_name: DialectName,
    wrapper: Wrapper,
    case_insensitive: bool,
}

impl DialectRunner {
    /// 对齐 Java: `DialectRunner(Dialect)`。
    #[must_use]
    pub fn new(pool: SqlitePool, dialect_name: DialectName, wrapper: Wrapper) -> Self {
        Self {
            inner: SqlConnRunner::create(pool),
            dialect_name,
            wrapper,
            case_insensitive: false,
        }
    }

    /// 从实现了 `Dialect` 的类型构造。
    #[must_use]
    pub fn from_dialect(pool: SqlitePool, dialect: &impl DialectTrait) -> Self {
        Self::new(pool, dialect.dialect_name(), dialect.wrapper())
    }

    /// 对齐 Java: `getDialect` —— 返回方言名。
    #[must_use]
    pub fn dialect_name(&self) -> DialectName {
        self.dialect_name
    }

    /// 对齐 Java: `setDialect`。
    pub fn set_dialect_name(&mut self, name: DialectName) -> &mut Self {
        self.dialect_name = name;
        self
    }

    /// 对齐 Java: `setWrapper`。
    pub fn set_wrapper(&mut self, wrapper: Wrapper) -> &mut Self {
        self.wrapper = wrapper;
        self
    }

    /// 对齐 Java: `setCaseInsensitive`。
    pub fn set_case_insensitive(&mut self, value: bool) -> &mut Self {
        self.case_insensitive = value;
        self
    }

    /// 当前 Wrapper。
    #[must_use]
    pub fn wrapper(&self) -> Wrapper {
        self.wrapper
    }

    /// 委托 insert。
    pub async fn insert(&self, entity: &Entity) -> DbResult<u64> {
        let _ = self.case_insensitive;
        self.inner.insert(entity).await
    }

    /// 委托 find。
    pub async fn find(&self, where_entity: &Entity) -> DbResult<Vec<Entity>> {
        self.inner.find(where_entity).await
    }

    /// 委托 count。
    pub async fn count(&self, where_entity: &Entity) -> DbResult<u64> {
        self.inner.count(where_entity).await
    }

    /// 委托 page。
    pub async fn page(
        &self,
        sql: &str,
        page: &HutoolPage,
        params: &[Value],
    ) -> DbResult<PageResult> {
        self.inner.page(sql, page, params).await
    }

    /// 委托 upsert。
    pub async fn upsert(&self, entity: &Entity, unique_field: &str) -> DbResult<u64> {
        self.inner.abstract_db().upsert(entity, unique_field).await
    }

    /// 委托 del。
    pub async fn del(&self, table: &str, field: &str, value: impl Into<Value>) -> DbResult<u64> {
        self.inner.abstract_db().del(table, field, value).await
    }

    /// 委托 update。
    pub async fn update(&self, set: &Entity, where_entity: &Entity) -> DbResult<u64> {
        self.inner.abstract_db().update(set, where_entity).await
    }
}
