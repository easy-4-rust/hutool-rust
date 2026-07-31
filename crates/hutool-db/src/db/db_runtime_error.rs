//! Db 门面 —— 对齐 Hutool `cn.hutool.db.Db`（SQLx SQLite 实现）。

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
