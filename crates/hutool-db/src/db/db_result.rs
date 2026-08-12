//! Db 门面 —— 对齐 Hutool `cn.hutool.db.Db`（SQLx SQLite 实现）。

use super::db_runtime_error::DbRuntimeError;

/// Db 操作结果类型别名。
pub type DbResult<T> = Result<T, DbRuntimeError>;
