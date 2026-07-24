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

/// 对齐 Hutool `TransactionLevel`。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransactionLevel {
    /// 不对事务隔离级别做约束（由驱动默认）。
    None,
    /// READ UNCOMMITTED。
    ReadUncommitted,
    /// READ COMMITTED。
    ReadCommitted,
    /// REPEATABLE READ。
    RepeatableRead,
    /// SERIALIZABLE。
    Serializable,
}

impl TransactionLevel {
    /// JDBC 隔离级别常量风格数值（仅作元数据）。
    #[must_use]
    pub fn jdbc_level(self) -> i32 {
        match self {
            Self::None => 0,
            Self::ReadUncommitted => 1,
            Self::ReadCommitted => 2,
            Self::RepeatableRead => 4,
            Self::Serializable => 8,
        }
    }
}
