//! Db 门面 —— 对齐 Hutool `cn.hutool.db.Db`（SQLx SQLite 实现）。

use crate::entity::Entity;
use crate::hutool_page::HutoolPage;
use crate::page_result::PageResult;
use crate::sql::condition::LikeType;
use crate::sql::named_sql::NamedSql;
use crate::sql::sql_util::{build_conditions, build_like_value, remove_outer_order_by};
use crate::sql::{Condition, SqlBuilder};
use serde_json::Value;
use sqlx::{Column, Row, SqlitePool, TypeInfo};
use std::collections::HashMap;

/// 数据库操作错误。
#[derive(Debug, thiserror::Error)]
pub enum DbRuntimeError {
    /// SQLx 执行错误。
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    /// 业务错误。
    #[error("{0}")]
    Message(String),
}

use super::{bind_value, memory_pool, row_to_entity, seed_hutool_user_fixture};
