//! Database primitives and optional `SQLx` engines.

#![forbid(unsafe_code)]

pub mod active_entity;
pub mod dao_template;
pub mod db;
pub mod dialect;
pub mod ds;
pub mod entity;
pub mod global_db_config;
pub mod handler;
pub mod hutool_page;
pub mod jdbc_wrapper;
pub mod meta;
pub mod meta_types;
pub mod nosql;
pub mod page_result;
pub mod prelude;
pub mod runner;
pub mod session;
pub mod sql;
pub mod thread_local_connection;
pub mod wrapper;

use std::time::Duration;
use thiserror::Error;

pub use active_entity::ActiveEntity;
pub use dao_template::{DaoOperations, DaoTemplate};
pub use db::{memory_pool, seed_hutool_user_fixture, Db, DbResult, DbRuntimeError};
pub use global_db_config::{GlobalDbConfig, LogLevel};
pub use jdbc_wrapper::{AbstractDataSource, ConnectionWraper, DbWrapperError, StatementWrapper};
pub use thread_local_connection::{GroupedConnection, ThreadLocalConnection};
pub use dialect::{
    identify_driver, identify_driver_from_text, AnsiSqlDialect, Dialect, DialectName, DmDialect,
    H2Dialect, HanaDialect, MysqlDialect, OracleDialect, PhoenixDialect, PostgresqlDialect,
    SqlServer2012Dialect, Sqlite3Dialect,
};
pub use ds::{
    AbstractDsFactory, BeeDsFactory, C3p0DsFactory, DataSourceWrapper, DbConfig, DbSetting,
    DbcpDsFactory, DruidDsFactory, DsFactory, HikariDsFactory, PooledDataSource, PooledDsFactory,
    SimpleDataSource, SimpleDsFactory, TomcatDsFactory,
};
pub use entity::Entity;
pub use handler::{
    BeanHandler, BeanListHandler, EntityHandler, EntityListHandler, EntitySetHandler, HandleHelper,
    NumberHandler, PageResultHandler, RsHandler, StringHandler, ValueListHandler,
};
pub use hutool_page::HutoolPage;
pub use meta::{
    get_column_names, get_table_meta, get_table_meta_or_err, get_tables, Column, ColumnIndexInfo,
    IndexInfo, JdbcType, Table, TableType,
};
pub use nosql::{MongoDs, RedisDs};
pub use page_result::PageResult;
pub use runner::{AbstractDb, DialectRunner, SqlConnRunner, SqlExecutor, TransactionLevel};
pub use session::Session;
pub use sql::{
    build_conditions, build_like_value, format_sql, is_in_clause, remove_outer_order_by,
    Condition, ConditionBuilder, ConditionGroup, ConditionValue, Direction, Join, LikeType,
    LogicalOperator, NamedSql, Order, Query, SqlBuilder, SqlLog,
};
pub use wrapper::Wrapper;

/// Database utility errors.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum DbError {
    /// Page numbers start at one.
    #[error("page number must be greater than zero")]
    InvalidPage,
    /// Page size is outside the configured range.
    #[error("page size must be between 1 and {maximum}")]
    InvalidPageSize {
        /// Maximum accepted page size.
        maximum: u32,
    },
    /// Offset computation exceeded `u64`.
    #[error("page offset overflowed")]
    OffsetOverflow,
}

/// A validated one-based page request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageRequest {
    page: u64,
    size: u32,
}

impl PageRequest {
    /// Validates page number and requested size.
    pub fn new(page: u64, size: u32, maximum_size: u32) -> Result<Self, DbError> {
        if page == 0 {
            return Err(DbError::InvalidPage);
        }
        if size == 0 || size > maximum_size {
            return Err(DbError::InvalidPageSize {
                maximum: maximum_size,
            });
        }
        Ok(Self { page, size })
    }

    /// Returns the one-based page number.
    #[must_use]
    pub fn page(self) -> u64 {
        self.page
    }

    /// Returns the page size.
    #[must_use]
    pub fn size(self) -> u32 {
        self.size
    }

    /// Returns the SQL offset.
    pub fn offset(self) -> Result<u64, DbError> {
        (self.page - 1)
            .checked_mul(u64::from(self.size))
            .ok_or(DbError::OffsetOverflow)
    }
}

/// One page of records and total-count metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Page<T> {
    /// Current records.
    pub records: Vec<T>,
    /// One-based page number.
    pub page: u64,
    /// Requested page size.
    pub size: u32,
    /// Total matching records.
    pub total: u64,
}

impl<T> Page<T> {
    /// Creates page metadata from a validated request.
    #[must_use]
    pub fn new(records: Vec<T>, request: PageRequest, total: u64) -> Self {
        Self {
            records,
            page: request.page,
            size: request.size,
            total,
        }
    }

    /// 对齐 Java: `PageResult.getPage()` / Hutool Page 元数据。
    #[must_use]
    pub fn page_number(&self) -> u64 {
        self.page
    }

    /// 对齐 Java: `PageResult.getPageSize()`.
    #[must_use]
    pub fn page_size(&self) -> u32 {
        self.size
    }

    /// Returns the total page count using ceiling division.
    #[must_use]
    pub fn total_pages(&self) -> u64 {
        self.total.div_ceil(u64::from(self.size))
    }

    /// Returns whether a following page can exist.
    #[must_use]
    pub fn has_next(&self) -> bool {
        self.page < self.total_pages()
    }
}

/// Shared `SQLx` connection-pool policy.
#[derive(Debug, Clone, Copy)]
pub struct PoolConfig {
    /// Maximum open connections.
    pub max_connections: u32,
    /// Minimum idle connections.
    pub min_connections: u32,
    /// Maximum time waiting to acquire a connection.
    pub acquire_timeout: Duration,
    /// Maximum idle time before a connection is closed.
    pub idle_timeout: Option<Duration>,
    /// Maximum lifetime of a connection.
    pub max_lifetime: Option<Duration>,
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            max_connections: 20,
            min_connections: 1,
            acquire_timeout: Duration::from_secs(10),
            idle_timeout: Some(Duration::from_secs(600)),
            max_lifetime: Some(Duration::from_secs(1_800)),
        }
    }
}

/// `PostgreSQL` pool support.
#[cfg(feature = "postgres")]
pub mod postgres {
    use super::PoolConfig;
    pub use sqlx::{PgPool, Postgres};

    /// Connects a `PostgreSQL` pool with the shared production policy.
    pub async fn connect(url: &str, config: PoolConfig) -> Result<PgPool, sqlx::Error> {
        sqlx::postgres::PgPoolOptions::new()
            .max_connections(config.max_connections)
            .min_connections(config.min_connections)
            .acquire_timeout(config.acquire_timeout)
            .idle_timeout(config.idle_timeout)
            .max_lifetime(config.max_lifetime)
            .connect(url)
            .await
    }
}

/// `MySQL` pool support.
#[cfg(feature = "mysql")]
pub mod mysql {
    use super::PoolConfig;
    pub use sqlx::{MySql, MySqlPool};

    /// Connects a `MySQL` pool with the shared production policy.
    pub async fn connect(url: &str, config: PoolConfig) -> Result<MySqlPool, sqlx::Error> {
        sqlx::mysql::MySqlPoolOptions::new()
            .max_connections(config.max_connections)
            .min_connections(config.min_connections)
            .acquire_timeout(config.acquire_timeout)
            .idle_timeout(config.idle_timeout)
            .max_lifetime(config.max_lifetime)
            .connect(url)
            .await
    }
}

/// `SQLite` pool support.
#[cfg(feature = "sqlite")]
pub mod sqlite {
    use super::PoolConfig;
    pub use sqlx::{Sqlite, SqlitePool};

    /// Connects a `SQLite` pool with the shared production policy.
    pub async fn connect(url: &str, config: PoolConfig) -> Result<SqlitePool, sqlx::Error> {
        sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(config.max_connections)
            .min_connections(config.min_connections)
            .acquire_timeout(config.acquire_timeout)
            .idle_timeout(config.idle_timeout)
            .max_lifetime(config.max_lifetime)
            .connect(url)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pagination_is_validated_and_uses_ceiling_division() {
        assert_eq!(PageRequest::new(0, 10, 100), Err(DbError::InvalidPage));
        let request = PageRequest::new(2, 10, 100).unwrap();
        assert_eq!(request.offset().unwrap(), 10);
        let page = Page::new(vec![1, 2], request, 21);
        assert_eq!(page.total_pages(), 3);
        assert!(page.has_next());
    }

    #[cfg(feature = "sqlite")]
    #[tokio::test]
    async fn sqlite_pool_executes_real_transactional_sql() {
        let pool = sqlite::connect(
            "sqlite::memory:",
            PoolConfig {
                max_connections: 1,
                min_connections: 1,
                ..PoolConfig::default()
            },
        )
        .await
        .unwrap();
        sqlx::query("CREATE TABLE item (id INTEGER PRIMARY KEY, name TEXT NOT NULL)")
            .execute(&pool)
            .await
            .unwrap();
        let mut transaction = pool.begin().await.unwrap();
        sqlx::query("INSERT INTO item (name) VALUES (?)")
            .bind("hutool")
            .execute(&mut *transaction)
            .await
            .unwrap();
        transaction.commit().await.unwrap();
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM item")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(count.0, 1);
    }

    #[cfg(feature = "postgres")]
    #[tokio::test]
    async fn postgres_pool_executes_real_transactional_sql() {
        let Ok(url) = std::env::var("HITOOL_TEST_POSTGRES_URL") else {
            return;
        };
        let pool = postgres::connect(
            &url,
            PoolConfig {
                max_connections: 1,
                min_connections: 0,
                ..PoolConfig::default()
            },
        )
        .await
        .unwrap();
        sqlx::query("CREATE TEMP TABLE hutool_item (id BIGSERIAL PRIMARY KEY, name TEXT NOT NULL)")
            .execute(&pool)
            .await
            .unwrap();
        let mut transaction = pool.begin().await.unwrap();
        sqlx::query("INSERT INTO hutool_item (name) VALUES ($1)")
            .bind("hutool")
            .execute(&mut *transaction)
            .await
            .unwrap();
        transaction.commit().await.unwrap();
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM hutool_item")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(count.0, 1);
    }

    #[cfg(feature = "mysql")]
    #[tokio::test]
    async fn mysql_pool_executes_real_transactional_sql() {
        let Ok(url) = std::env::var("HITOOL_TEST_MYSQL_URL") else {
            return;
        };
        let pool = mysql::connect(
            &url,
            PoolConfig {
                max_connections: 1,
                min_connections: 0,
                ..PoolConfig::default()
            },
        )
        .await
        .unwrap();
        sqlx::query(
            "CREATE TEMPORARY TABLE hutool_item (id BIGINT AUTO_INCREMENT PRIMARY KEY, name TEXT NOT NULL)",
        )
        .execute(&pool)
        .await
        .unwrap();
        let mut transaction = pool.begin().await.unwrap();
        sqlx::query("INSERT INTO hutool_item (name) VALUES (?)")
            .bind("hutool")
            .execute(&mut *transaction)
            .await
            .unwrap();
        transaction.commit().await.unwrap();
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM hutool_item")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(count.0, 1);
    }
}
