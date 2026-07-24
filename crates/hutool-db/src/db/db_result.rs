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

use super::db_runtime_error::DbRuntimeError;

pub type DbResult<T> = Result<T, DbRuntimeError>;

use super::{bind_value, memory_pool, row_to_entity, seed_hutool_user_fixture};
