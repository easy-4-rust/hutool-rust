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
    /// 执行查询 SQL 并返回结果。
    fn execute_query_sql(&self, sql: &str) -> Result<Box<dyn Any>, DbWrapperError>;
    /// 执行更新 SQL 并返回影响行数。
    fn execute_update_sql(&self, sql: &str) -> Result<i64, DbWrapperError>;
    /// 关闭 Statement。
    fn close(&self) -> Result<(), DbWrapperError>;
    /// 读取字段大小上限。
    fn get_max_field_size(&self) -> Result<i32, DbWrapperError>;
    /// 设置字段大小上限。
    fn set_max_field_size(&self, max: i32) -> Result<(), DbWrapperError>;
    /// 读取最大行数。
    fn get_max_rows(&self) -> Result<i32, DbWrapperError>;
    /// 设置最大行数。
    fn set_max_rows(&self, max: i32) -> Result<(), DbWrapperError>;
    /// 设置转义处理开关。
    fn set_escape_processing(&self, enable: bool) -> Result<(), DbWrapperError>;
    /// 读取查询超时（秒）。
    fn get_query_timeout(&self) -> Result<i32, DbWrapperError>;
    /// 设置查询超时（秒）。
    fn set_query_timeout(&self, seconds: i32) -> Result<(), DbWrapperError>;
    /// 取消正在执行的语句。
    fn cancel(&self) -> Result<(), DbWrapperError>;
    /// 设置游标名称。
    fn set_cursor_name(&self, name: &str) -> Result<(), DbWrapperError>;
    /// 执行任意 SQL。
    fn execute_sql(&self, sql: &str) -> Result<bool, DbWrapperError>;
    /// 读取更新计数。
    fn get_update_count(&self) -> Result<i64, DbWrapperError>;
    /// 设置抓取方向。
    fn set_fetch_direction(&self, direction: i32) -> Result<(), DbWrapperError>;
    /// 读取抓取方向。
    fn get_fetch_direction(&self) -> Result<i32, DbWrapperError>;
    /// 设置抓取大小。
    fn set_fetch_size(&self, rows: i32) -> Result<(), DbWrapperError>;
    /// 读取抓取大小。
    fn get_fetch_size(&self) -> Result<i32, DbWrapperError>;
    /// 追加批处理 SQL。
    fn add_batch(&self, sql: &str) -> Result<(), DbWrapperError>;
    /// 清空批处理队列。
    fn clear_batch(&self) -> Result<(), DbWrapperError>;
    /// 执行批处理。
    fn execute_batch(&self) -> Result<Vec<i64>, DbWrapperError>;
    /// 是否已关闭。
    fn is_closed(&self) -> bool;
    /// 设置是否可池化。
    fn set_poolable(&self, poolable: bool) -> Result<(), DbWrapperError>;
    /// 是否可池化。
    fn is_poolable(&self) -> bool;

    // ─── PreparedStatement 参数绑定 ───
    /// 绑定 NULL 值。
    fn set_null(&self, parameter_index: i32, sql_type: i32) -> Result<(), DbWrapperError>;
    /// 绑定布尔值。
    fn set_boolean(&self, parameter_index: i32, x: bool) -> Result<(), DbWrapperError>;
    /// 绑定字节值。
    fn set_byte(&self, parameter_index: i32, x: i8) -> Result<(), DbWrapperError>;
    /// 绑定短整型。
    fn set_short(&self, parameter_index: i32, x: i16) -> Result<(), DbWrapperError>;
    /// 绑定整型。
    fn set_int(&self, parameter_index: i32, x: i32) -> Result<(), DbWrapperError>;
    /// 绑定长整型。
    fn set_long(&self, parameter_index: i32, x: i64) -> Result<(), DbWrapperError>;
    /// 绑定浮点值。
    fn set_float(&self, parameter_index: i32, x: f32) -> Result<(), DbWrapperError>;
    /// 绑定双精度值。
    fn set_double(&self, parameter_index: i32, x: f64) -> Result<(), DbWrapperError>;
    /// 绑定字符串。
    fn set_string(&self, parameter_index: i32, x: &str) -> Result<(), DbWrapperError>;
    /// 绑定字节数组。
    fn set_bytes(&self, parameter_index: i32, x: &[u8]) -> Result<(), DbWrapperError>;
    /// 清空参数。
    fn clear_parameters(&self) -> Result<(), DbWrapperError>;
    /// 绑定任意对象。
    fn set_object(&self, parameter_index: i32, x: &dyn Any) -> Result<(), DbWrapperError>;
    /// 执行预编译语句。
    fn execute_prepared(&self) -> Result<bool, DbWrapperError>;
    /// 执行预编译查询。
    fn execute_query_prepared(&self) -> Result<Box<dyn Any>, DbWrapperError>;
    /// 执行预编译更新。
    fn execute_update_prepared(&self) -> Result<i64, DbWrapperError>;
}
