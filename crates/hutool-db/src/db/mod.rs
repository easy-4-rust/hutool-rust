//! Db 门面 —— 对齐 Hutool `cn.hutool.db.Db`（SQLx SQLite 实现）。

use crate::entity::Entity;
use serde_json::Value;
use sqlx::{Column, Row, SqlitePool, TypeInfo};

mod db;
mod db_result;
mod db_runtime_error;

pub use db::Db;
pub use db_result::DbResult;
pub use db_runtime_error::DbRuntimeError;

fn bind_value<'q>(
    query: sqlx::query::Query<'q, sqlx::Sqlite, sqlx::sqlite::SqliteArguments>,
    value: &Value,
) -> sqlx::query::Query<'q, sqlx::Sqlite, sqlx::sqlite::SqliteArguments> {
    match value {
        Value::Null => query.bind(None::<String>),
        Value::Bool(v) => query.bind(*v),
        Value::Number(n) => {
            if let Some(v) = n.as_i64() {
                query.bind(v)
            } else if let Some(v) = n.as_f64() {
                query.bind(v)
            } else {
                query.bind(n.to_string())
            }
        }
        Value::String(s) => query.bind(s.clone()),
        Value::Array(_) | Value::Object(_) => query.bind(value.to_string()),
    }
}

fn row_to_entity(row: sqlx::sqlite::SqliteRow) -> Entity {
    let mut entity = Entity::create();
    for column in row.columns() {
        let name = column.name().to_string();
        let value = match column.type_info().name() {
            "INTEGER" | "INT" => Value::Number(row.get::<i64, _>(name.as_str()).into()),
            "REAL" | "FLOAT" => Value::Number(
                serde_json::Number::from_f64(row.get::<f64, _>(name.as_str())).unwrap_or(0.into()),
            ),
            _ => {
                let s: Option<String> = row.try_get(name.as_str()).ok();
                s.map(Value::String).unwrap_or(Value::Null)
            }
        };
        entity.set_value(name, value);
    }
    entity
}

/// 初始化 hutool 测试用 `user` / `user_1` 表与数据（对齐 Hutool 测试夹具）。
pub async fn seed_hutool_user_fixture(pool: &SqlitePool) -> DbResult<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS \"user\" (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            name TEXT,
            age INTEGER,
            birthday TEXT,
            gender INTEGER
        )",
    )
    .execute(pool)
    .await?;
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS user_1 (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            name TEXT,
            age INTEGER,
            birthday TEXT,
            gender INTEGER
        )",
    )
    .execute(pool)
    .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS user_1_birthday_index ON user_1 (birthday)")
        .execute(pool)
        .await?;
    sqlx::query(
        "CREATE UNIQUE INDEX IF NOT EXISTS user_1_birthday_name_uindex ON user_1 (birthday, name)",
    )
    .execute(pool)
    .await?;
    sqlx::query("DELETE FROM \"user\"").execute(pool).await?;
    for (id, name, age, birthday, gender) in [
        (1i64, "张三", 12i64, None, None),
        (2, "王五", 18, None, None),
        (9, "张三", 12, Some("19900112"), Some(1i64)),
        (12, "unitTestUser", 76, None, None),
    ] {
        sqlx::query(
            "INSERT INTO \"user\" (id, name, age, birthday, gender) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(id)
        .bind(name)
        .bind(age)
        .bind(birthday)
        .bind(gender)
        .execute(pool)
        .await?;
    }
    Ok(())
}

/// 创建内存 `SQLite` 连接池并灌入测试夹具数据。
pub async fn memory_pool() -> DbResult<SqlitePool> {
    let pool = crate::sqlite::connect(
        "sqlite::memory:",
        crate::PoolConfig {
            max_connections: 5,
            min_connections: 1,
            ..crate::PoolConfig::default()
        },
    )
    .await?;
    seed_hutool_user_fixture(&pool).await?;
    Ok(pool)
}
