//! Db 运行器门面 —— 对齐 Hutool `AbstractDb` / `SqlConnRunner` / `DialectRunner` / `SqlExecutor`。
//!
//! 均委托现有 `Db`（SQLx pool），避免 JDBC Connection / Statement 全局状态。

mod abstract_db;
mod dialect_runner;
mod sql_conn_runner;
mod sql_executor;
mod transaction_level;

pub use abstract_db::AbstractDb;
pub use dialect_runner::DialectRunner;
pub use sql_conn_runner::SqlConnRunner;
pub use sql_executor::SqlExecutor;
pub use transaction_level::TransactionLevel;
