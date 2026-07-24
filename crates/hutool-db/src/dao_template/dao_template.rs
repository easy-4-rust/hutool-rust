//! DaoTemplate facade，对齐 hutool 的 `cn.hutool.db.DaoTemplate`。
//!
//! 提供 CRUD 模板：add/del/update/get/find/page/count/exist 等 25 个方法。
//! 具体数据库操作依赖 Db（基于 SQLx），属于 unsafe-to-copy 的 JDBC 部分。

use crate::entity::Entity;
use crate::page_result::PageResult;
use crate::hutool_page::HutoolPage;
use crate::DbResult;

use super::dao_operations::DaoOperations;

/// DAO 模板，对齐 `cn.hutool.db.DaoTemplate`。
///
/// Java 版通过 Db + DataSource 操作；Rust 版用 trait + Db 引用。
/// 调用方传入 `dyn DaoOperations` 实现具体查询逻辑。
pub struct DaoTemplate {
    table_name: String,
    primary_key_field: String,
}

impl DaoTemplate {
    /// 对齐 `DaoTemplate(String tableName)`
    pub fn new(table_name: &str) -> Self {
        Self {
            table_name: table_name.to_string(),
            primary_key_field: "id".to_string(),
        }
    }

    /// 对齐 `DaoTemplate(String tableName, String primaryKeyField)`
    pub fn with_primary_key(table_name: &str, primary_key_field: &str) -> Self {
        Self {
            table_name: table_name.to_string(),
            primary_key_field: primary_key_field.to_string(),
        }
    }

    /// 对齐 `DaoTemplate(String tableName, DataSource ds)` — Rust 用 DSFactory trait
    pub fn with_datasource_name(table_name: &str, _ds_name: &str) -> Self {
        Self::new(table_name)
    }

    /// 对齐 `DaoTemplate(String, String, DataSource)`
    pub fn with_pk_and_ds(
        table_name: &str,
        primary_key_field: &str,
        _ds_name: &str,
    ) -> Self {
        Self::with_primary_key(table_name, primary_key_field)
    }

    /// 对齐 `DaoTemplate(String, String, Db)`
    pub fn with_pk_and_db(
        table_name: &str,
        primary_key_field: &str,
        _db: &crate::Db,
    ) -> Self {
        Self::with_primary_key(table_name, primary_key_field)
    }

    /// 表名 getter
    pub fn table_name(&self) -> &str {
        &self.table_name
    }

    /// 主键字段 getter
    pub fn primary_key_field(&self) -> &str {
        &self.primary_key_field
    }

    // ─── CRUD：具体操作委托到 DaoOperations trait ───

    /// 对齐 `add(Entity)`：插入
    pub fn add(&self, entity: &Entity, ops: &dyn DaoOperations) -> DbResult<i64> {
        ops.add(&self.table_name, entity)
    }

    /// 对齐 `addForGeneratedKeys(Entity)`
    pub fn add_for_generated_keys(
        &self,
        entity: &Entity,
        ops: &dyn DaoOperations,
    ) -> DbResult<Vec<serde_json::Value>> {
        ops.add_for_generated_keys(&self.table_name, entity)
    }

    /// 对齐 `addForGeneratedKey(Entity)`
    pub fn add_for_generated_key(&self, entity: &Entity, ops: &dyn DaoOperations) -> DbResult<i64> {
        ops.add_for_generated_key(&self.table_name, entity)
    }

    /// 对齐 `del(T pk)`
    pub fn del<T: ToString>(&self, pk: T, ops: &dyn DaoOperations) -> DbResult<i64> {
        ops.del_by_field(&self.table_name, &self.primary_key_field, &pk.to_string())
    }

    /// 对齐 `del(String field, T value)`
    pub fn del_by<T: ToString>(
        &self,
        field: &str,
        value: T,
        ops: &dyn DaoOperations,
    ) -> DbResult<i64> {
        ops.del_by_field(&self.table_name, field, &value.to_string())
    }

    /// 对齐 `del(Entity where)`
    pub fn del_by_entity(
        &self,
        where_entity: &Entity,
        ops: &dyn DaoOperations,
    ) -> DbResult<i64> {
        ops.del_by_entity(&self.table_name, where_entity)
    }

    /// 对齐 `update(Entity record, Entity where)`
    pub fn update(
        &self,
        record: &Entity,
        where_entity: &Entity,
        ops: &dyn DaoOperations,
    ) -> DbResult<i64> {
        ops.update(&self.table_name, record, where_entity)
    }

    /// 对齐 `update(Entity entity)`：按主键更新
    pub fn update_by_pk(&self, entity: &Entity, ops: &dyn DaoOperations) -> DbResult<i64> {
        ops.update_by_pk(&self.table_name, &self.primary_key_field, entity)
    }

    /// 对齐 `addOrUpdate(Entity entity)`
    pub fn add_or_update(&self, entity: &Entity, ops: &dyn DaoOperations) -> DbResult<i64> {
        ops.add_or_update(&self.table_name, &self.primary_key_field, entity)
    }

    /// 对齐 `get(T pk)`
    pub fn get<T: ToString>(&self, pk: T, ops: &dyn DaoOperations) -> DbResult<Option<Entity>> {
        ops.get_by_field(&self.table_name, &self.primary_key_field, &pk.to_string())
    }

    /// 对齐 `get(String field, T value)`
    pub fn get_by<T: ToString>(
        &self,
        field: &str,
        value: T,
        ops: &dyn DaoOperations,
    ) -> DbResult<Option<Entity>> {
        ops.get_by_field(&self.table_name, field, &value.to_string())
    }

    /// 对齐 `get(Entity where)`
    pub fn get_by_entity(
        &self,
        where_entity: &Entity,
        ops: &dyn DaoOperations,
    ) -> DbResult<Option<Entity>> {
        ops.get_by_entity(&self.table_name, where_entity)
    }

    /// 对齐 `find(String field, T value)`
    pub fn find<T: ToString>(
        &self,
        field: &str,
        value: T,
        ops: &dyn DaoOperations,
    ) -> DbResult<Vec<Entity>> {
        ops.find_by_field(&self.table_name, field, &value.to_string())
    }

    /// 对齐 `findAll()`
    pub fn find_all(&self, ops: &dyn DaoOperations) -> DbResult<Vec<Entity>> {
        ops.find_all(&self.table_name)
    }

    /// 对齐 `find(Entity where)`
    pub fn find_by_entity(
        &self,
        where_entity: &Entity,
        ops: &dyn DaoOperations,
    ) -> DbResult<Vec<Entity>> {
        ops.find_by_entity(&self.table_name, where_entity)
    }

    /// 对齐 `findBySql(String sql, Object... params)`
    pub fn find_by_sql(
        &self,
        sql: &str,
        params: &[serde_json::Value],
        ops: &dyn DaoOperations,
    ) -> DbResult<Vec<Entity>> {
        ops.find_by_sql(&self.table_name, sql, params)
    }

    /// 对齐 `page(Entity where, Page page, String... selectFields)`
    pub fn page(
        &self,
        where_entity: &Entity,
        page: &HutoolPage,
        select_fields: &[&str],
        ops: &dyn DaoOperations,
    ) -> DbResult<PageResult> {
        ops.page(&self.table_name, where_entity, page, select_fields)
    }

    /// 对齐 `page(Entity where, Page page)`
    pub fn page_default(
        &self,
        where_entity: &Entity,
        page: &HutoolPage,
        ops: &dyn DaoOperations,
    ) -> DbResult<PageResult> {
        ops.page(&self.table_name, where_entity, page, &[])
    }

    /// 对齐 `count(Entity where)`
    pub fn count(&self, where_entity: &Entity, ops: &dyn DaoOperations) -> DbResult<i64> {
        ops.count(&self.table_name, where_entity)
    }

    /// 对齐 `exist(Entity where)`
    pub fn exist(&self, where_entity: &Entity, ops: &dyn DaoOperations) -> DbResult<bool> {
        ops.exist(&self.table_name, where_entity)
    }
}
