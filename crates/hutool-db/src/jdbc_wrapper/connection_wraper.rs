//! JDBC Wrapper facade，对齐 hutool 的 `cn.hutool.db.sql.StatementWrapper`、
//! `cn.hutool.db.ds.pooled.ConnectionWraper`、`cn.hutool.db.ds.simple.AbstractDataSource`。
//!
//! **仅提供 trait 抽象 + 类型占位**。具体 JDBC 操作（基于 `java.sql.PreparedStatement`
//! / `java.sql.Connection`）是 `javax_sql_spi` unsafe-to-copy，Rust 用 SQLx 替代。
//!
//! 这些 trait 让 hutool 的 JDBC API 在 Rust 端"有迹可循"，方便迁移用户理解 API 形状。

use super::db_wrapper_error::DbWrapperError;
use super::statement_wrapper::StatementWrapper;

/// JDBC Connection 包装器 trait，对齐 `cn.hutool.db.ds.pooled.ConnectionWraper`。
///
/// Java 实现 `Connection`；Rust 用 trait 提供 API 形状。具体实现需要 SQLx Pool。
pub trait ConnectionWraper: Send + Sync {
    /// 创建 Statement。
    fn create_statement(&self) -> Result<Box<dyn StatementWrapper>, DbWrapperError>;
    /// 预编译 SQL。
    fn prepare_statement(&self, sql: &str) -> Result<Box<dyn StatementWrapper>, DbWrapperError>;
    /// 设置自动提交。
    fn set_auto_commit(&self, auto_commit: bool) -> Result<(), DbWrapperError>;
    /// 读取自动提交。
    fn get_auto_commit(&self) -> Result<bool, DbWrapperError>;
    /// 提交事务。
    fn commit(&self) -> Result<(), DbWrapperError>;
    /// 回滚事务。
    fn rollback(&self) -> Result<(), DbWrapperError>;
    /// 设置只读模式。
    fn set_read_only(&self, read_only: bool) -> Result<(), DbWrapperError>;
    /// 读取只读模式。
    fn is_read_only(&self) -> Result<bool, DbWrapperError>;
    /// 设置当前目录。
    fn set_catalog(&self, catalog: &str) -> Result<(), DbWrapperError>;
    /// 读取当前目录。
    fn get_catalog(&self) -> Result<String, DbWrapperError>;
    /// 设置事务隔离级别。
    fn set_transaction_isolation(&self, level: i32) -> Result<(), DbWrapperError>;
    /// 读取事务隔离级别。
    fn get_transaction_isolation(&self) -> Result<i32, DbWrapperError>;
    /// 清除警告。
    fn clear_warnings(&self) -> Result<(), DbWrapperError>;
    /// 校验连接是否有效。
    fn is_valid(&self, timeout_seconds: i32) -> bool;
    /// 关闭连接。
    fn close(&self) -> Result<(), DbWrapperError>;
}
