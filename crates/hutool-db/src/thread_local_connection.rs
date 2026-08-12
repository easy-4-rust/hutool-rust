//! ThreadLocalConnection facade，对齐 hutool 的 `cn.hutool.db.ThreadLocalConnection`。
//!
//! 基于 `thread_local!` 提供 thread-local Connection 管理。
//! 具体 Connection 类型（Java JDBC Connection）是 unsafe-to-copy，Rust 用 trait object 替代。
//!
//! **注意**：由于 Rust 借用规则限制，`get()` 返回 `bool`/`has()` 而非 `Box<dyn Any>`。
//! 用户应使用 `with_connection(ds_name, |conn| { ... })` 闭包模式访问。

use std::any::Any;
use std::collections::HashMap;

use crate::DbResult;
use crate::db::DbRuntimeError;

/// 全局 thread-local connection 持有者，对齐 `cn.hutool.db.ThreadLocalConnection`。
///
/// Java 版用 `static ThreadLocal<GroupedConnection>` + 每个线程一个 HashMap<DataSource, Connection>。
/// Rust 版用 `thread_local!` + `RefCell<GroupedConnection>`。
pub struct ThreadLocalConnection;

thread_local! {
    /// 当前线程的 grouped connection 集合
    static CURRENT: std::cell::RefCell<Option<GroupedConnection>> = const { std::cell::RefCell::new(None) };
}

impl ThreadLocalConnection {
    /// 对齐 `ThreadLocalConnection.get(DataSource ds)` 的存在性检查
    ///
    /// 返回 `true` 表示当前线程已绑定指定 ds_name 的 connection。
    /// （Java 返回 Connection 对象；Rust 因借用规则限制改为 has()）
    pub fn has(ds_name: &str) -> bool {
        CURRENT.with(|c| {
            c.borrow()
                .as_ref()
                .map(|g| g.connections.contains_key(ds_name))
                .unwrap_or(false)
        })
    }

    /// 对齐 `ThreadLocalConnection.get(DataSource ds)` — Rust 版返回 Result 标记
    ///
    /// 由于 `thread_local!` 不允许返回借用对象，这里返回 `Ok(())` 表示存在，
    /// `Err` 表示不存在。用户应配合 `with_connection` 闭包模式使用。
    pub fn get(ds_name: &str) -> DbResult<()> {
        if Self::has(ds_name) {
            Ok(())
        } else {
            Err(DbRuntimeError::Message(format!(
                "No thread-local connection for datasource: {}",
                ds_name
            )))
        }
    }

    /// 对齐 `ThreadLocalConnection.close(DataSource ds)`：关闭当前线程 + 指定 DS 的 Connection
    pub fn close(ds_name: &str) -> DbResult<()> {
        CURRENT.with(|c| {
            let mut borrowed = c.borrow_mut();
            if let Some(grouped) = borrowed.as_mut() {
                grouped.connections.remove(ds_name);
            }
            Ok(())
        })
    }

    /// 内部：在当前线程注入一个 connection（测试用 / 框架装配用）
    pub fn put(ds_name: &str, conn: Box<dyn Any + Send + Sync>) -> DbResult<()> {
        CURRENT.with(|c| {
            let mut borrowed = c.borrow_mut();
            if borrowed.is_none() {
                *borrowed = Some(GroupedConnection::new());
            }
            if let Some(grouped) = borrowed.as_mut() {
                grouped.connections.insert(ds_name.to_string(), conn);
            }
            Ok(())
        })
    }

    /// 内部：检查是否为空（对齐 `GroupedConnection.isEmpty()`）
    pub fn is_empty() -> bool {
        CURRENT.with(|c| {
            c.borrow()
                .as_ref()
                .map(|g| g.connections.is_empty())
                .unwrap_or(true)
        })
    }

    /// 闭包模式：在闭包中访问指定 ds_name 的 connection
    pub fn with_connection<R, F>(ds_name: &str, f: F) -> DbResult<R>
    where
        F: FnOnce(Option<&Box<dyn Any + Send + Sync>>) -> R,
    {
        CURRENT.with(|c| {
            let borrowed = c.borrow();
            let conn = borrowed.as_ref().and_then(|g| g.connections.get(ds_name));
            Ok(f(conn))
        })
    }
}

/// 分组连接集合，对齐 `cn.hutool.db.ThreadLocalConnection.GroupedConnection`。
///
/// 每个 GroupedConnection 关联一组 DataSource → Connection 映射。
pub struct GroupedConnection {
    pub(crate) connections: HashMap<String, Box<dyn Any + Send + Sync>>,
}

impl GroupedConnection {
    /// 对齐 `GroupedConnection()`
    pub fn new() -> Self {
        Self {
            connections: HashMap::new(),
        }
    }

    /// 对齐 `GroupedConnection.get(DataSource ds)` — 返回是否存在
    pub fn has(&self, ds_name: &str) -> bool {
        self.connections.contains_key(ds_name)
    }

    /// 对齐 `GroupedConnection.close(DataSource ds)`
    pub fn close(&mut self, ds_name: &str) -> DbResult<()> {
        self.connections.remove(ds_name);
        Ok(())
    }

    /// 对齐 `GroupedConnection.isEmpty()`
    pub fn is_empty(&self) -> bool {
        self.connections.is_empty()
    }
}

impl Default for GroupedConnection {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thread_local_connection_empty_by_default() {
        // 在新线程中测试以确保 clean state
        let handle = std::thread::spawn(|| ThreadLocalConnection::is_empty());
        let is_empty = handle.join().unwrap();
        assert!(is_empty);
    }

    #[test]
    fn test_thread_local_connection_put_and_has() {
        let handle = std::thread::spawn(|| {
            // put
            ThreadLocalConnection::put("main", Box::new(42i32)).unwrap();
            assert!(!ThreadLocalConnection::is_empty());
            // has
            assert!(ThreadLocalConnection::has("main"));
            assert!(!ThreadLocalConnection::has("missing"));
            // get returns Ok if exists
            let r = ThreadLocalConnection::get("main");
            assert!(r.is_ok());
            // close
            ThreadLocalConnection::close("main").unwrap();
            assert!(!ThreadLocalConnection::has("main"));
            assert!(ThreadLocalConnection::is_empty());
        });
        handle.join().unwrap();
    }

    #[test]
    fn test_thread_local_connection_get_missing() {
        let handle = std::thread::spawn(|| {
            let r = ThreadLocalConnection::get("missing");
            assert!(r.is_err());
        });
        handle.join().unwrap();
    }

    #[test]
    fn test_thread_local_connection_with_connection() {
        let handle = std::thread::spawn(|| {
            ThreadLocalConnection::put("main", Box::new(42i32)).unwrap();
            let result: i32 = ThreadLocalConnection::with_connection("main", |c| {
                c.and_then(|b| b.downcast_ref::<i32>().copied())
                    .unwrap_or(0)
            })
            .unwrap();
            assert_eq!(result, 42);
        });
        handle.join().unwrap();
    }

    #[test]
    fn test_grouped_connection_new() {
        let g = GroupedConnection::new();
        assert!(g.is_empty());
    }

    #[test]
    fn test_grouped_connection_has_missing() {
        let g = GroupedConnection::new();
        assert!(!g.has("any"));
    }
}
