//! Db 门面 —— 对齐 Hutool `cn.hutool.db.Db`（SQLx SQLite 实现）。

use crate::entity::Entity;
use crate::hutool_page::HutoolPage;
use crate::page_result::PageResult;
use crate::sql::condition::LikeType;
use crate::sql::named_sql::NamedSql;
use crate::sql::sql_util::{build_conditions, build_like_value, remove_outer_order_by};
use crate::sql::{Condition, SqlBuilder};
use serde_json::Value;
use sqlx::{AssertSqlSafe, Row, SqlitePool};
use std::collections::HashMap;

use super::db_result::DbResult;
use super::db_runtime_error::DbRuntimeError;

/// 对齐 Hutool `Db`。
#[derive(Debug, Clone)]
pub struct Db {
    pool: SqlitePool,
}

impl Db {
    /// 从连接池构造。
    #[must_use]
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// 对齐 Java: `Db.use()`.
    #[must_use]
    pub fn use_pool(pool: SqlitePool) -> Self {
        Self::new(pool)
    }

    /// 对齐 Java: `Db.query(String, Object...)`.
    pub async fn query(&self, sql: &str, params: &[Value]) -> DbResult<Vec<Entity>> {
        let mut query = sqlx::query(AssertSqlSafe(sql));
        for param in params {
            query = bind_value(query, param);
        }
        let rows = query.fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(row_to_entity).collect())
    }

    /// 对齐 Java: `Db.query(String, Map<String,Object>)`.
    pub async fn query_named(
        &self,
        sql: &str,
        param_map: &HashMap<String, Value>,
    ) -> DbResult<Vec<Entity>> {
        let named = NamedSql::new(sql, param_map);
        self.query(named.sql(), named.params()).await
    }

    /// 对齐 Java: `Db.find(Entity)`.
    pub async fn find(&self, where_entity: &Entity) -> DbResult<Vec<Entity>> {
        self.find_fields(where_entity.field_names(), where_entity)
            .await
    }

    /// 对齐 Java: `Db.find(Collection<String> fields, Entity where)`.
    pub async fn find_fields(
        &self,
        fields: &[String],
        where_entity: &Entity,
    ) -> DbResult<Vec<Entity>> {
        let table = where_entity
            .table_name()
            .ok_or_else(|| DbRuntimeError::Message("table name required".into()))?;
        let conditions = build_conditions(where_entity);
        let select = if fields.is_empty() {
            "*".to_string()
        } else {
            fields.join(",")
        };
        let mut builder = SqlBuilder::create();
        builder
            .select([select.as_str()])
            .from(format!("\"{table}\""));
        if !conditions.is_empty() {
            builder.where_conditions(&conditions);
        }
        self.query(&builder.build(), builder.param_values()).await
    }

    /// 对齐 Java: `Db.findAll(String)`.
    pub async fn find_all_table(&self, table: &str) -> DbResult<Vec<Entity>> {
        self.find(&Entity::create_table(table)).await
    }

    /// 对齐 Java: `Db.findAll(Entity)`.
    pub async fn find_all(&self, where_entity: &Entity) -> DbResult<Vec<Entity>> {
        self.find(where_entity).await
    }

    /// 对齐 Java: `Db.findBy(String, Condition...)`.
    pub async fn find_by(&self, table: &str, conditions: &[Condition]) -> DbResult<Vec<Entity>> {
        let mut entity = Entity::create_table(table);
        for condition in conditions {
            entity.set_condition(condition.field(), condition.clone());
        }
        self.find(&entity).await
    }

    /// 对齐 Java: `Db.findLike(...)`.
    pub async fn find_like(
        &self,
        table: &str,
        field: &str,
        value: &str,
        like_type: LikeType,
    ) -> DbResult<Vec<Entity>> {
        let like_expr = build_like_value(value, like_type, true);
        self.find(&Entity::create_table(table).with(field, Value::String(like_expr)))
            .await
    }

    /// 对齐 Java: `Db.page(Entity, int, int)`.
    pub async fn page_entity(
        &self,
        where_entity: &Entity,
        page: u32,
        size: u32,
    ) -> DbResult<Vec<Entity>> {
        let table = where_entity
            .table_name()
            .ok_or_else(|| DbRuntimeError::Message("table name required".into()))?;
        let conditions = build_conditions(where_entity);
        let mut builder = SqlBuilder::create();
        builder.select(["*"]).from(format!("\"{table}\""));
        if !conditions.is_empty() {
            builder.where_conditions(&conditions);
        }
        let offset = page.saturating_mul(size);
        let sql = format!("{} LIMIT {} OFFSET {}", builder.build(), size, offset);
        self.query(&sql, builder.param_values()).await
    }

    /// 对齐 Java: `Db.page(String, Page)`.
    pub async fn page_sql(
        &self,
        sql: &str,
        page: &HutoolPage,
        params: &[Value],
    ) -> DbResult<Vec<Entity>> {
        let offset = page.page_number().saturating_mul(page.page_size());
        let paged = format!("{} LIMIT {} OFFSET {}", sql, page.page_size(), offset);
        self.query(&paged, params).await
    }

    /// 对齐 Java: `Db.page(String, Page, Object...)`.
    pub async fn page_sql_with_params(
        &self,
        sql: &str,
        page: &HutoolPage,
        params: &[Value],
    ) -> DbResult<PageResult> {
        let total = self.count_sql(sql, params).await?;
        let records = self.page_sql(sql, page, params).await?;
        Ok(PageResult::new(
            page.page_number(),
            page.page_size(),
            total,
            records,
        ))
    }

    /// 对齐 Java: `Db.count(String)`.
    pub async fn count_sql(&self, sql: &str, params: &[Value]) -> DbResult<u64> {
        let stripped = remove_outer_order_by(sql);
        let count_sql = format!("SELECT COUNT(*) AS c FROM ({stripped}) tmp_count");
        let row = {
            let mut query = sqlx::query(AssertSqlSafe(count_sql));
            for param in params {
                query = bind_value(query, param);
            }
            query.fetch_one(&self.pool).await?
        };
        Ok(row.get::<i64, _>(0) as u64)
    }

    /// 对齐 Java: `Db.count(Entity)`.
    pub async fn count_entity(&self, where_entity: &Entity) -> DbResult<u64> {
        let table = where_entity
            .table_name()
            .ok_or_else(|| DbRuntimeError::Message("table name required".into()))?;
        let conditions = build_conditions(where_entity);
        let mut builder = SqlBuilder::create();
        builder.select(["*"]).from(format!("\"{table}\""));
        if !conditions.is_empty() {
            builder.where_conditions(&conditions);
        }
        self.count_sql(&builder.build(), builder.param_values())
            .await
    }

    /// 对齐 Java: `Db.get`.
    pub async fn get(
        &self,
        table: &str,
        field: &str,
        value: impl Into<Value>,
    ) -> DbResult<Option<Entity>> {
        let rows = self
            .find(&Entity::create_table(table).with(field, value.into()))
            .await?;
        Ok(rows.into_iter().next())
    }

    /// 对齐 Java: `Db.insert(Entity)`.
    pub async fn insert(&self, entity: &Entity) -> DbResult<u64> {
        let mut builder = SqlBuilder::create();
        builder.insert(entity);
        let sql = builder.build();
        let result = {
            let mut query = sqlx::query(AssertSqlSafe(sql));
            for param in builder.param_values() {
                query = bind_value(query, param);
            }
            query.execute(&self.pool).await?
        };
        Ok(result.last_insert_rowid() as u64)
    }

    /// 对齐 Java: `Db.update(Entity set, Entity where)`.
    pub async fn update(&self, set: &Entity, where_entity: &Entity) -> DbResult<u64> {
        let table = where_entity
            .table_name()
            .or(set.table_name())
            .ok_or_else(|| DbRuntimeError::Message("table name required".into()))?;
        let mut set_entity = set.clone();
        set_entity.set_table_name(table);
        let conditions = build_conditions(where_entity);
        let mut builder = SqlBuilder::create();
        builder.update(&set_entity);
        if !conditions.is_empty() {
            builder.where_conditions(&conditions);
        }
        let sql = builder.build();
        let result = {
            let mut query = sqlx::query(AssertSqlSafe(sql));
            for param in builder.param_values() {
                query = bind_value(query, param);
            }
            query.execute(&self.pool).await?
        };
        Ok(result.rows_affected())
    }

    /// 对齐 Java: `Db.del`.
    pub async fn del(&self, table: &str, field: &str, value: impl Into<Value>) -> DbResult<u64> {
        // SQLx 0.9 要求动态 SQL 显式声明安全性；标识符先按 SQLite 规则转义双引号。
        let table = table.replace('"', "\"\"");
        let field = field.replace('"', "\"\"");
        let sql = format!("DELETE FROM \"{table}\" WHERE \"{field}\" = ?");
        let value = value.into();
        let result = bind_value(sqlx::query(AssertSqlSafe(sql)), &value)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected())
    }

    /// 对齐 Java: `Db.upsert` —— SQLite `INSERT OR REPLACE`.
    pub async fn upsert(&self, entity: &Entity, unique_field: &str) -> DbResult<u64> {
        let table = entity
            .table_name()
            .ok_or_else(|| DbRuntimeError::Message("table name required".into()))?;
        let table = table.replace('"', "\"\"");
        let mut cols = Vec::new();
        let mut placeholders = Vec::new();
        let mut params = Vec::new();
        for (field, value) in entity.iter() {
            let field = field.replace('"', "\"\"");
            cols.push(format!("\"{field}\""));
            placeholders.push('?');
            params.push(value.clone());
        }
        let sql = format!(
            "INSERT OR REPLACE INTO \"{table}\" ({}) VALUES ({})",
            cols.join(", "),
            std::iter::repeat("?")
                .take(params.len())
                .collect::<Vec<_>>()
                .join(", ")
        );
        let _ = unique_field;
        let mut query = sqlx::query(AssertSqlSafe(sql));
        for param in &params {
            query = bind_value(query, param);
        }
        let result = query.execute(&self.pool).await?;
        Ok(result.rows_affected())
    }

    /// 对齐 Java: `Db.tx`.
    pub async fn tx<F, Fut>(&self, func: F) -> DbResult<()>
    where
        F: FnOnce(Db) -> Fut,
        Fut: std::future::Future<Output = DbResult<()>>,
    {
        func(Db::new(self.pool.clone())).await?;
        Ok(())
    }

    /// 对齐 Java: `Db.execute`.
    pub async fn execute(&self, sql: &str) -> DbResult<()> {
        sqlx::query(AssertSqlSafe(sql)).execute(&self.pool).await?;
        Ok(())
    }

    /// 返回底层连接池。
    #[must_use]
    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
}

use super::{bind_value, row_to_entity};
