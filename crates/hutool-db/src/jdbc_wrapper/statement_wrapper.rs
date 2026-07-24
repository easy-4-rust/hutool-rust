//! JDBC Wrapper facade，对齐 hutool 的 `cn.hutool.db.sql.StatementWrapper`、
//! `cn.hutool.db.ds.pooled.ConnectionWraper`、`cn.hutool.db.ds.simple.AbstractDataSource`。
//!
//! **仅提供 trait 抽象 + 类型占位**。具体 JDBC 操作（基于 `java.sql.PreparedStatement`
//! / `java.sql.Connection`）是 `javax_sql_spi` unsafe-to-copy，Rust 用 SQLx 替代。
//!
//! 这些 trait 让 hutool 的 JDBC API 在 Rust 端"有迹可循"，方便迁移用户理解 API 形状。

use std::any::Any;

use super::db_wrapper_error::DbWrapperError;

/// JDBC Statement 包装器 trait，对齐 `cn.hutool.db.sql.StatementWrapper`。
///
/// Java 继承 `PreparedStatementWrapper implements PreparedStatement`；
/// Rust 用 trait 提供完整 JDBC API 形状。具体实现需要 SQLx 或 rusqlite。
pub trait StatementWrapper: Send + Sync {
    // ─── 通用 Statement 方法 ───
    fn execute_query_sql(&self, sql: &str) -> Result<Box<dyn Any>, DbWrapperError>;
    fn execute_update_sql(&self, sql: &str) -> Result<i64, DbWrapperError>;
    fn close(&self) -> Result<(), DbWrapperError>;
    fn get_max_field_size(&self) -> Result<i32, DbWrapperError>;
    fn set_max_field_size(&self, max: i32) -> Result<(), DbWrapperError>;
    fn get_max_rows(&self) -> Result<i32, DbWrapperError>;
    fn set_max_rows(&self, max: i32) -> Result<(), DbWrapperError>;
    fn set_escape_processing(&self, enable: bool) -> Result<(), DbWrapperError>;
    fn get_query_timeout(&self) -> Result<i32, DbWrapperError>;
    fn set_query_timeout(&self, seconds: i32) -> Result<(), DbWrapperError>;
    fn cancel(&self) -> Result<(), DbWrapperError>;
    fn set_cursor_name(&self, name: &str) -> Result<(), DbWrapperError>;
    fn execute_sql(&self, sql: &str) -> Result<bool, DbWrapperError>;
    fn get_update_count(&self) -> Result<i64, DbWrapperError>;
    fn set_fetch_direction(&self, direction: i32) -> Result<(), DbWrapperError>;
    fn get_fetch_direction(&self) -> Result<i32, DbWrapperError>;
    fn set_fetch_size(&self, rows: i32) -> Result<(), DbWrapperError>;
    fn get_fetch_size(&self) -> Result<i32, DbWrapperError>;
    fn add_batch(&self, sql: &str) -> Result<(), DbWrapperError>;
    fn clear_batch(&self) -> Result<(), DbWrapperError>;
    fn execute_batch(&self) -> Result<Vec<i64>, DbWrapperError>;
    fn is_closed(&self) -> bool;
    fn set_poolable(&self, poolable: bool) -> Result<(), DbWrapperError>;
    fn is_poolable(&self) -> bool;

    // ─── PreparedStatement 参数绑定 ───
    fn set_null(&self, parameter_index: i32, sql_type: i32) -> Result<(), DbWrapperError>;
    fn set_boolean(&self, parameter_index: i32, x: bool) -> Result<(), DbWrapperError>;
    fn set_byte(&self, parameter_index: i32, x: i8) -> Result<(), DbWrapperError>;
    fn set_short(&self, parameter_index: i32, x: i16) -> Result<(), DbWrapperError>;
    fn set_int(&self, parameter_index: i32, x: i32) -> Result<(), DbWrapperError>;
    fn set_long(&self, parameter_index: i32, x: i64) -> Result<(), DbWrapperError>;
    fn set_float(&self, parameter_index: i32, x: f32) -> Result<(), DbWrapperError>;
    fn set_double(&self, parameter_index: i32, x: f64) -> Result<(), DbWrapperError>;
    fn set_string(&self, parameter_index: i32, x: &str) -> Result<(), DbWrapperError>;
    fn set_bytes(&self, parameter_index: i32, x: &[u8]) -> Result<(), DbWrapperError>;
    fn clear_parameters(&self) -> Result<(), DbWrapperError>;
    fn set_object(&self, parameter_index: i32, x: &dyn Any) -> Result<(), DbWrapperError>;
    fn execute_prepared(&self) -> Result<bool, DbWrapperError>;
    fn execute_query_prepared(&self) -> Result<Box<dyn Any>, DbWrapperError>;
    fn execute_update_prepared(&self) -> Result<i64, DbWrapperError>;
}
