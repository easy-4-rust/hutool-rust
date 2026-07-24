//! JDBC Wrapper facade，对齐 hutool 的 `cn.hutool.db.sql.StatementWrapper`、
//! `cn.hutool.db.ds.pooled.ConnectionWraper`、`cn.hutool.db.ds.simple.AbstractDataSource`。
//!
//! **仅提供 trait 抽象 + 类型占位**。具体 JDBC 操作（基于 `java.sql.PreparedStatement`
//! / `java.sql.Connection`）是 `javax_sql_spi` unsafe-to-copy，Rust 用 SQLx 替代。
//!
//! 这些 trait 让 hutool 的 JDBC API 在 Rust 端"有迹可循"，方便迁移用户理解 API 形状。

use std::any::Any;

mod statement_wrapper;
mod connection_wraper;
mod abstract_data_source;
mod db_wrapper_error;

pub use statement_wrapper::StatementWrapper;
pub use connection_wraper::ConnectionWraper;
pub use abstract_data_source::AbstractDataSource;
pub use db_wrapper_error::DbWrapperError;
