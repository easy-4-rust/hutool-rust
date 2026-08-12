//! 集成测试共享 fixture —— SQLite 内存库 + Hutool `user` 表 seed。

use hutool_db::{Db, memory_pool};

/// 返回已 seed `user` 表的 `Db`（`sqlite::memory:`）。
pub async fn test_db() -> Db {
    Db::new(memory_pool().await.expect("memory pool"))
}
