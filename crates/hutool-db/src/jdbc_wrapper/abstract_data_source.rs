//! JDBC Wrapper facade，对齐 hutool 的 `cn.hutool.db.sql.StatementWrapper`、
//! `cn.hutool.db.ds.pooled.ConnectionWraper`、`cn.hutool.db.ds.simple.AbstractDataSource`。
//!
//! **仅提供 trait 抽象 + 类型占位**。具体 JDBC 操作（基于 `java.sql.PreparedStatement`
//! / `java.sql.Connection`）是 `javax_sql_spi` unsafe-to-copy，Rust 用 SQLx 替代。
//!
//! 这些 trait 让 hutool 的 JDBC API 在 Rust 端"有迹可循"，方便迁移用户理解 API 形状。

use std::any::Any;

use super::db_wrapper_error::DbWrapperError;

/// 抽象 DataSource，对齐 `cn.hutool.db.ds.simple.AbstractDataSource`。
///
/// Java 实现 `javax.sql.DataSource`；Rust 用 trait 提供形状。
pub trait AbstractDataSource: Send + Sync {
    fn set_login_timeout(&self, seconds: i32) -> Result<(), DbWrapperError>;
    fn get_login_timeout(&self) -> Result<i32, DbWrapperError>;
    fn unwrap(&self) -> Option<Box<dyn Any>>;
    fn is_wrapper_for(&self) -> bool;
}
