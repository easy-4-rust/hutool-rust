//! JDBC Wrapper facade，对齐 hutool 的 `cn.hutool.db.sql.StatementWrapper`、
//! `cn.hutool.db.ds.pooled.ConnectionWraper`、`cn.hutool.db.ds.simple.AbstractDataSource`。
//!
//! **仅提供 trait 抽象 + 类型占位**。具体 JDBC 操作（基于 `java.sql.PreparedStatement`
//! / `java.sql.Connection`）是 `javax_sql_spi` unsafe-to-copy，Rust 用 SQLx 替代。
//!
//! 这些 trait 让 hutool 的 JDBC API 在 Rust 端"有迹可循"，方便迁移用户理解 API 形状。

use std::any::Any;

use super::db_wrapper_error::DbWrapperError;
use super::statement_wrapper::StatementWrapper;

/// JDBC Connection 包装器 trait，对齐 `cn.hutool.db.ds.pooled.ConnectionWraper`。
///
/// Java 实现 `Connection`；Rust 用 trait 提供 API 形状。具体实现需要 SQLx Pool。
pub trait ConnectionWraper: Send + Sync {
    fn create_statement(&self) -> Result<Box<dyn StatementWrapper>, DbWrapperError>;
    fn prepare_statement(&self, sql: &str) -> Result<Box<dyn StatementWrapper>, DbWrapperError>;
    fn set_auto_commit(&self, auto_commit: bool) -> Result<(), DbWrapperError>;
    fn get_auto_commit(&self) -> Result<bool, DbWrapperError>;
    fn commit(&self) -> Result<(), DbWrapperError>;
    fn rollback(&self) -> Result<(), DbWrapperError>;
    fn set_read_only(&self, read_only: bool) -> Result<(), DbWrapperError>;
    fn is_read_only(&self) -> Result<bool, DbWrapperError>;
    fn set_catalog(&self, catalog: &str) -> Result<(), DbWrapperError>;
    fn get_catalog(&self) -> Result<String, DbWrapperError>;
    fn set_transaction_isolation(&self, level: i32) -> Result<(), DbWrapperError>;
    fn get_transaction_isolation(&self) -> Result<i32, DbWrapperError>;
    fn clear_warnings(&self) -> Result<(), DbWrapperError>;
    fn is_valid(&self, timeout_seconds: i32) -> bool;
    fn close(&self) -> Result<(), DbWrapperError>;
}
