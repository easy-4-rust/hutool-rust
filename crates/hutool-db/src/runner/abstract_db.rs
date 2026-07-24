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

/// 对齐 Hutool `AbstractDb`：与 `Db` 同构的显式门面。
#[derive(Debug, Clone)]
pub struct AbstractDb {
    inner: Db,
    case_insensitive: bool,
    wrapper: Option<Wrapper>,
}

impl AbstractDb {
    /// 对齐 Java: `AbstractDb(DataSource)` —— 使用已注入的 pool。
    #[must_use]
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            inner: Db::new(pool),
            case_insensitive: false,
            wrapper: None,
        }
    }

    /// 从已有 `Db` 包装。
    #[must_use]
    pub fn from_db(db: Db) -> Self {
        Self {
            inner: db,
            case_insensitive: false,
            wrapper: None,
        }
    }

    /// 返回内部 `Db`。
    #[must_use]
    pub fn db(&self) -> &Db {
        &self.inner
    }

    /// 对齐 Java: `getDs()` —— 返回底层 pool。
    #[must_use]
    pub fn pool(&self) -> &SqlitePool {
        self.inner.pool()
    }

    /// 对齐 Java: `setCaseInsensitive`。
    pub fn set_case_insensitive(&mut self, value: bool) -> &mut Self {
        self.case_insensitive = value;
        self
    }

    /// 对齐 Java: `setWrapper(Character)` / `setWrapper(Wrapper)`。
    pub fn set_wrapper(&mut self, wrapper: Wrapper) -> &mut Self {
        self.wrapper = Some(wrapper);
        self
    }

    /// 对齐 Java: `disableWrapper()`。
    pub fn disable_wrapper(&mut self) -> &mut Self {
        self.wrapper = None;
        self
    }

    /// 对齐 Java: `query`。
    pub async fn query(&self, sql: &str, params: &[Value]) -> DbResult<Vec<Entity>> {
        self.inner.query(sql, params).await
    }

    /// 对齐 Java: `query` named。
    pub async fn query_named(
        &self,
        sql: &str,
        param_map: &HashMap<String, Value>,
    ) -> DbResult<Vec<Entity>> {
        self.inner.query_named(sql, param_map).await
    }

    /// 对齐 Java: `find`。
    pub async fn find(&self, where_entity: &Entity) -> DbResult<Vec<Entity>> {
        self.inner.find(where_entity).await
    }

    /// 对齐 Java: `findAll`。
    pub async fn find_all(&self, table: &str) -> DbResult<Vec<Entity>> {
        self.inner.find_all_table(table).await
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
        self.inner.count_entity(where_entity).await
    }

    /// 对齐 Java: `page`。
    pub async fn page(
        &self,
        sql: &str,
        page: &HutoolPage,
        params: &[Value],
    ) -> DbResult<PageResult> {
        self.inner.page_sql_with_params(sql, page, params).await
    }

    /// 对齐 Java: `insert`。
    pub async fn insert(&self, entity: &Entity) -> DbResult<u64> {
        self.inner.insert(entity).await
    }

    /// 对齐 Java: `update`。
    pub async fn update(&self, set: &Entity, where_entity: &Entity) -> DbResult<u64> {
        self.inner.update(set, where_entity).await
    }

    /// 对齐 Java: `del`。
    pub async fn del(&self, table: &str, field: &str, value: impl Into<Value>) -> DbResult<u64> {
        self.inner.del(table, field, value).await
    }

    /// 对齐 Java: `upsert`。
    pub async fn upsert(&self, entity: &Entity, unique_field: &str) -> DbResult<u64> {
        self.inner.upsert(entity, unique_field).await
    }

    /// 对齐 Java: `execute`。
    pub async fn execute(&self, sql: &str) -> DbResult<()> {
        self.inner.execute(sql).await
    }

    /// 对齐 Java: `get`。
    pub async fn get(
        &self,
        table: &str,
        field: &str,
        value: impl Into<Value>,
    ) -> DbResult<Option<Entity>> {
        self.inner.get(table, field, value).await
    }

    /// 当前大小写敏感标志。
    #[must_use]
    pub fn case_insensitive(&self) -> bool {
        self.case_insensitive
    }

    /// 当前 Wrapper。
    #[must_use]
    pub fn wrapper(&self) -> Option<Wrapper> {
        self.wrapper
    }
}
