//! DaoTemplate facade，对齐 hutool 的 `cn.hutool.db.DaoTemplate`。
//!
//! 提供 CRUD 模板：add/del/update/get/find/page/count/exist 等 25 个方法。
//! 具体数据库操作依赖 Db（基于 SQLx），属于 unsafe-to-copy 的 JDBC 部分。

use crate::entity::Entity;
use crate::page_result::PageResult;
use crate::hutool_page::HutoolPage;
use crate::DbResult;

/// DAO 操作 trait，DaoTemplate 通过此 trait 执行具体数据库操作。
///
/// 用户/框架提供具体实现（基于 SQLx 或其他 ORM）。
pub trait DaoOperations {
    /// 新增一条记录，返回主键。
    fn add(&self, table: &str, entity: &Entity) -> DbResult<i64>;
    /// 新增并返回全部生成键。
    fn add_for_generated_keys(
        &self,
        table: &str,
        entity: &Entity,
    ) -> DbResult<Vec<serde_json::Value>>;
    /// 新增并返回第一个生成键。
    fn add_for_generated_key(&self, table: &str, entity: &Entity) -> DbResult<i64>;
    /// 按字段值删除。
    fn del_by_field(&self, table: &str, field: &str, value: &str) -> DbResult<i64>;
    /// 按实体条件删除。
    fn del_by_entity(&self, table: &str, where_entity: &Entity) -> DbResult<i64>;
    /// 按条件更新记录。
    fn update(
        &self,
        table: &str,
        record: &Entity,
        where_entity: &Entity,
    ) -> DbResult<i64>;
    /// 按主键更新。
    fn update_by_pk(&self, table: &str, pk_field: &str, entity: &Entity) -> DbResult<i64>;
    /// 存在则更新，否则新增。
    fn add_or_update(&self, table: &str, pk_field: &str, entity: &Entity) -> DbResult<i64>;
    /// 按字段取值查询单条。
    fn get_by_field(
        &self,
        table: &str,
        field: &str,
        value: &str,
    ) -> DbResult<Option<Entity>>;
    /// 按实体条件查询单条。
    fn get_by_entity(&self, table: &str, where_entity: &Entity) -> DbResult<Option<Entity>>;
    /// 按字段取值查询列表。
    fn find_by_field(&self, table: &str, field: &str, value: &str) -> DbResult<Vec<Entity>>;
    /// 查询全表。
    fn find_all(&self, table: &str) -> DbResult<Vec<Entity>>;
    /// 按实体条件查询列表。
    fn find_by_entity(&self, table: &str, where_entity: &Entity) -> DbResult<Vec<Entity>>;
    /// 按 SQL 查询列表。
    fn find_by_sql(
        &self,
        table: &str,
        sql: &str,
        params: &[serde_json::Value],
    ) -> DbResult<Vec<Entity>>;
    /// 分页查询。
    fn page(
        &self,
        table: &str,
        where_entity: &Entity,
        page: &HutoolPage,
        select_fields: &[&str],
    ) -> DbResult<PageResult>;
    /// 统计记录数。
    fn count(&self, table: &str, where_entity: &Entity) -> DbResult<i64>;
    /// 判断记录是否存在。
    fn exist(&self, table: &str, where_entity: &Entity) -> DbResult<bool>;
}
