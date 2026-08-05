//! 元数据工具 —— 对齐 Hutool `cn.hutool.db.meta.MetaUtil`（SQLite 子集）。

use crate::db::{DbResult, DbRuntimeError};
use sqlx::{AssertSqlSafe, Row, SqlitePool};
use std::collections::HashSet;

pub use crate::meta_types::{Column, ColumnIndexInfo, IndexInfo, JdbcType, TableType};

/// 表元数据。
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Table {
    pk_names: HashSet<String>,
    index_count: usize,
}

impl Table {
    /// 主键列名集合。
    #[must_use]
    pub fn pk_names(&self) -> &HashSet<String> {
        &self.pk_names
    }

    /// 索引数量。
    #[must_use]
    pub fn index_info_list_len(&self) -> usize {
        self.index_count
    }
}

/// 对齐 Java: `MetaUtil.getTables(DataSource)`.
pub async fn get_tables(pool: &SqlitePool) -> DbResult<Vec<String>> {
    let rows = sqlx::query(
        "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' ORDER BY name",
    )
    .fetch_all(pool)
    .await?;
    Ok(rows
        .into_iter()
        .map(|row| row.get::<String, _>(0))
        .collect())
}

/// 对齐 Java: `MetaUtil.getTableMeta(DataSource, String)`.
pub async fn get_table_meta(pool: &SqlitePool, table: &str) -> DbResult<Table> {
    let table = table.replace('"', "\"\"");
    let rows = sqlx::query(AssertSqlSafe(format!("PRAGMA table_info(\"{table}\")")))
        .fetch_all(pool)
        .await?;
    let mut pk_names = HashSet::new();
    for row in rows {
        let name: String = row.get(1);
        let pk: i64 = row.get(5);
        if pk > 0 {
            pk_names.insert(name);
        }
    }
    let indexes = sqlx::query(AssertSqlSafe(format!("PRAGMA index_list(\"{table}\")")))
        .fetch_all(pool)
        .await?;
    Ok(Table {
        pk_names,
        index_count: indexes.len(),
    })
}

/// 对齐 Java: `MetaUtil.getColumnNames(DataSource, String)`.
pub async fn get_column_names(pool: &SqlitePool, table: &str) -> DbResult<Vec<String>> {
    let table = table.replace('"', "\"\"");
    let rows = sqlx::query(AssertSqlSafe(format!("PRAGMA table_info(\"{table}\")")))
        .fetch_all(pool)
        .await?;
    Ok(rows
        .into_iter()
        .map(|row| row.get::<String, _>(1))
        .collect())
}

/// 对齐 Java: 表不存在时抛出 `DbRuntimeException`。
pub async fn get_table_meta_or_err(pool: &SqlitePool, table: &str) -> DbResult<Table> {
    let exists: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name = ?")
            .bind(table)
            .fetch_one(pool)
            .await?;
    if exists.0 == 0 {
        return Err(DbRuntimeError::Message(format!(
            "Table [{table}] not exists"
        )));
    }
    get_table_meta(pool, table).await
}
