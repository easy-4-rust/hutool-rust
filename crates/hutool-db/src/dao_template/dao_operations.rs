//! DaoTemplate facade，对齐 hutool 的 `cn.hutool.db.DaoTemplate`。
//!
//! 提供 CRUD 模板：add/del/update/get/find/page/count/exist 等 25 个方法。
//! 具体数据库操作依赖 Db（基于 SQLx），属于 unsafe-to-copy 的 JDBC 部分。

use crate::entity::Entity;
use crate::page_result::PageResult;
use crate::hutool_page::HutoolPage;
use crate::DbResult;

use super::dao_template::DaoTemplate;

/// DAO 操作 trait，DaoTemplate 通过此 trait 执行具体数据库操作。
///
/// 用户/框架提供具体实现（基于 SQLx 或其他 ORM）。
pub trait DaoOperations {
    fn add(&self, table: &str, entity: &Entity) -> DbResult<i64>;
    fn add_for_generated_keys(
        &self,
        table: &str,
        entity: &Entity,
    ) -> DbResult<Vec<serde_json::Value>>;
    fn add_for_generated_key(&self, table: &str, entity: &Entity) -> DbResult<i64>;
    fn del_by_field(&self, table: &str, field: &str, value: &str) -> DbResult<i64>;
    fn del_by_entity(&self, table: &str, where_entity: &Entity) -> DbResult<i64>;
    fn update(
        &self,
        table: &str,
        record: &Entity,
        where_entity: &Entity,
    ) -> DbResult<i64>;
    fn update_by_pk(&self, table: &str, pk_field: &str, entity: &Entity) -> DbResult<i64>;
    fn add_or_update(&self, table: &str, pk_field: &str, entity: &Entity) -> DbResult<i64>;
    fn get_by_field(
        &self,
        table: &str,
        field: &str,
        value: &str,
    ) -> DbResult<Option<Entity>>;
    fn get_by_entity(&self, table: &str, where_entity: &Entity) -> DbResult<Option<Entity>>;
    fn find_by_field(&self, table: &str, field: &str, value: &str) -> DbResult<Vec<Entity>>;
    fn find_all(&self, table: &str) -> DbResult<Vec<Entity>>;
    fn find_by_entity(&self, table: &str, where_entity: &Entity) -> DbResult<Vec<Entity>>;
    fn find_by_sql(
        &self,
        table: &str,
        sql: &str,
        params: &[serde_json::Value],
    ) -> DbResult<Vec<Entity>>;
    fn page(
        &self,
        table: &str,
        where_entity: &Entity,
        page: &HutoolPage,
        select_fields: &[&str],
    ) -> DbResult<PageResult>;
    fn count(&self, table: &str, where_entity: &Entity) -> DbResult<i64>;
    fn exist(&self, table: &str, where_entity: &Entity) -> DbResult<bool>;
}
