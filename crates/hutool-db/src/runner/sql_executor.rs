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

/// 对齐 Hutool `SqlExecutor`：静态风格执行入口（需显式传入 `Db`）。
pub struct SqlExecutor;

impl SqlExecutor {
    /// 对齐 Java: `execute(Connection, String, Object...)`。
    pub async fn execute(db: &Db, sql: &str) -> DbResult<()> {
        db.execute(sql).await
    }

    /// 对齐 Java: `query`。
    pub async fn query(db: &Db, sql: &str, params: &[Value]) -> DbResult<Vec<Entity>> {
        db.query(sql, params).await
    }

    /// 对齐 Java: `executeUpdate` —— 返回影响行数（DDL/DML 统一走 execute）。
    pub async fn execute_update(db: &Db, sql: &str) -> DbResult<()> {
        db.execute(sql).await
    }
}
