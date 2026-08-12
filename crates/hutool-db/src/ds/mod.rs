//! 数据源配置与工厂 —— 对齐 Hutool `cn.hutool.db.ds.*`（显式注入，无全局单例）。

mod factory;

pub use factory::{
    AbstractDsFactory, BeeDsFactory, C3p0DsFactory, DbcpDsFactory, DruidDsFactory, DsFactory,
    HikariDsFactory, PooledDsFactory, SimpleDsFactory, TomcatDsFactory,
};

use crate::PoolConfig;
use std::collections::HashMap;
use std::time::Duration;

/// 对齐 Hutool `DataSourceWrapper`（简化：保存 URL 与 driver）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataSourceWrapper {
    url: String,
    user: String,
    pass: String,
    driver: String,
}

impl DataSourceWrapper {
    /// 构造包装器。
    #[must_use]
    pub fn new(
        url: impl Into<String>,
        user: impl Into<String>,
        pass: impl Into<String>,
        driver: impl Into<String>,
    ) -> Self {
        Self {
            url: url.into(),
            user: user.into(),
            pass: pass.into(),
            driver: driver.into(),
        }
    }

    /// 对齐 Java: `getDriver()`.
    #[must_use]
    pub fn driver(&self) -> &str {
        &self.driver
    }

    /// 对齐 Java: `getRaw()` —— 返回 JDBC URL。
    #[must_use]
    pub fn raw_url(&self) -> &str {
        &self.url
    }

    /// 对齐 Java: `getUser()` / `getPassword()` for c3p0 test.
    #[must_use]
    pub fn user(&self) -> &str {
        &self.user
    }

    /// 返回密码。
    #[must_use]
    pub fn pass(&self) -> &str {
        &self.pass
    }
}

/// 对齐 Hutool `cn.hutool.db.ds.pooled.DbConfig`。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DbConfig {
    driver: String,
    url: String,
    user: String,
    pass: String,
    initial_size: u32,
    min_idle: u32,
    max_active: u32,
    max_wait_ms: u64,
    conn_props: HashMap<String, String>,
}

impl DbConfig {
    /// 对齐 Java: `DbConfig()`。
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// 对齐 Java: `DbConfig(String url, String user, String pass)`。
    #[must_use]
    pub fn of(url: impl Into<String>, user: impl Into<String>, pass: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            user: user.into(),
            pass: pass.into(),
            ..Self::default()
        }
    }

    /// 对齐 Java: `init(String url, String user, String pass)`。
    pub fn init(
        &mut self,
        url: impl Into<String>,
        user: impl Into<String>,
        pass: impl Into<String>,
    ) {
        self.url = url.into();
        self.user = user.into();
        self.pass = pass.into();
    }

    /// 对齐 Java: `getDriver()` / `setDriver`。
    #[must_use]
    pub fn driver(&self) -> &str {
        &self.driver
    }

    /// 设置 JDBC 驱动类名。
    pub fn set_driver(&mut self, driver: impl Into<String>) -> &mut Self {
        self.driver = driver.into();
        self
    }

    /// 对齐 Java: `getUrl()`。
    #[must_use]
    pub fn url(&self) -> &str {
        &self.url
    }

    /// 设置 URL。
    pub fn set_url(&mut self, url: impl Into<String>) -> &mut Self {
        self.url = url.into();
        self
    }

    /// 对齐 Java: `getUser()`。
    #[must_use]
    pub fn user(&self) -> &str {
        &self.user
    }

    /// 设置用户。
    pub fn set_user(&mut self, user: impl Into<String>) -> &mut Self {
        self.user = user.into();
        self
    }

    /// 对齐 Java: `getPass()`。
    #[must_use]
    pub fn pass(&self) -> &str {
        &self.pass
    }

    /// 设置密码。
    pub fn set_pass(&mut self, pass: impl Into<String>) -> &mut Self {
        self.pass = pass.into();
        self
    }

    /// 对齐 Java: `getInitialSize()`。
    #[must_use]
    pub fn initial_size(&self) -> u32 {
        self.initial_size
    }

    /// 设置初始连接数。
    pub fn set_initial_size(&mut self, size: u32) -> &mut Self {
        self.initial_size = size;
        self
    }

    /// 对齐 Java: `getMinIdle()`。
    #[must_use]
    pub fn min_idle(&self) -> u32 {
        self.min_idle
    }

    /// 设置最小空闲连接。
    pub fn set_min_idle(&mut self, size: u32) -> &mut Self {
        self.min_idle = size;
        self
    }

    /// 对齐 Java: `getMaxActive()`。
    #[must_use]
    pub fn max_active(&self) -> u32 {
        self.max_active
    }

    /// 设置最大活跃连接。
    pub fn set_max_active(&mut self, size: u32) -> &mut Self {
        self.max_active = size;
        self
    }

    /// 对齐 Java: `getMaxWait()`（毫秒）。
    #[must_use]
    pub fn max_wait_ms(&self) -> u64 {
        self.max_wait_ms
    }

    /// 设置获取连接最大等待毫秒。
    pub fn set_max_wait_ms(&mut self, ms: u64) -> &mut Self {
        self.max_wait_ms = ms;
        self
    }

    /// 对齐 Java: `getConnProps()`。
    #[must_use]
    pub fn conn_props(&self) -> &HashMap<String, String> {
        &self.conn_props
    }

    /// 对齐 Java: `setConnProps`。
    pub fn set_conn_props(&mut self, props: HashMap<String, String>) -> &mut Self {
        self.conn_props = props;
        self
    }

    /// 对齐 Java: `addConnProps`。
    pub fn add_conn_props(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> &mut Self {
        self.conn_props.insert(key.into(), value.into());
        self
    }

    /// 转换为 hutool `PoolConfig`（显式注入策略）。
    #[must_use]
    pub fn to_pool_config(&self) -> PoolConfig {
        PoolConfig {
            max_connections: self.max_active.max(1),
            min_connections: self.min_idle.min(self.max_active.max(1)),
            acquire_timeout: Duration::from_millis(self.max_wait_ms.max(1)),
            idle_timeout: Some(Duration::from_secs(600)),
            max_lifetime: Some(Duration::from_secs(1_800)),
        }
    }

    /// 转为 `DataSourceWrapper`。
    #[must_use]
    pub fn to_wrapper(&self) -> DataSourceWrapper {
        DataSourceWrapper::new(&self.url, &self.user, &self.pass, &self.driver)
    }
}

impl Default for DbConfig {
    fn default() -> Self {
        Self {
            driver: String::new(),
            url: String::new(),
            user: String::new(),
            pass: String::new(),
            initial_size: 0,
            min_idle: 1,
            max_active: 20,
            max_wait_ms: 10_000,
            conn_props: HashMap::new(),
        }
    }
}

/// 对齐 Hutool `DbSetting`：从键值配置构建 `DbConfig`（非全局 Setting 单例）。
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DbSetting {
    entries: HashMap<String, String>,
}

impl DbSetting {
    /// 对齐 Java: `DbSetting()`。
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// 写入配置项。
    pub fn put(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.entries.insert(key.into(), value.into());
        self
    }

    /// 对齐 Java: `getDbConfig(String group)` —— 按前缀读取。
    #[must_use]
    pub fn get_db_config(&self, group: &str) -> DbConfig {
        let prefix = if group.is_empty() {
            String::new()
        } else {
            format!("{group}.")
        };
        let get = |key: &str| {
            self.entries
                .get(&format!("{prefix}{key}"))
                .cloned()
                .unwrap_or_default()
        };
        let mut cfg = DbConfig::of(get("url"), get("user"), get("pass"));
        cfg.set_driver(get("driver"));
        if let Ok(v) = get("initialSize").parse() {
            cfg.set_initial_size(v);
        }
        if let Ok(v) = get("minIdle").parse() {
            cfg.set_min_idle(v);
        }
        if let Ok(v) = get("maxActive").parse() {
            cfg.set_max_active(v);
        }
        if let Ok(v) = get("maxWait").parse() {
            cfg.set_max_wait_ms(v);
        }
        cfg
    }
}

/// 对齐 Hutool `SimpleDataSource`：持有连接参数，不打开全局连接。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimpleDataSource {
    config: DbConfig,
}

impl SimpleDataSource {
    /// 对齐 Java: `getDataSource()` 静态工厂的本地等价。
    #[must_use]
    pub fn get_data_source(
        url: impl Into<String>,
        user: impl Into<String>,
        pass: impl Into<String>,
    ) -> Self {
        Self::new(url, user, pass)
    }

    /// 对齐 Java 构造。
    #[must_use]
    pub fn new(url: impl Into<String>, user: impl Into<String>, pass: impl Into<String>) -> Self {
        Self {
            config: DbConfig::of(url, user, pass),
        }
    }

    /// 从完整 `DbConfig` 构造。
    #[must_use]
    pub fn from_config(config: DbConfig) -> Self {
        Self { config }
    }

    /// 对齐 Java: `init`。
    pub fn init(
        &mut self,
        url: impl Into<String>,
        user: impl Into<String>,
        pass: impl Into<String>,
    ) {
        self.config.init(url, user, pass);
    }

    /// 对齐 Java: `getDriver()`。
    #[must_use]
    pub fn driver(&self) -> &str {
        self.config.driver()
    }

    /// 设置驱动。
    pub fn set_driver(&mut self, driver: impl Into<String>) -> &mut Self {
        self.config.set_driver(driver);
        self
    }

    /// 对齐 Java: `getUrl()`。
    #[must_use]
    pub fn url(&self) -> &str {
        self.config.url()
    }

    /// 设置 URL。
    pub fn set_url(&mut self, url: impl Into<String>) -> &mut Self {
        self.config.set_url(url);
        self
    }

    /// 对齐 Java: `getUser()`。
    #[must_use]
    pub fn user(&self) -> &str {
        self.config.user()
    }

    /// 设置用户。
    pub fn set_user(&mut self, user: impl Into<String>) -> &mut Self {
        self.config.set_user(user);
        self
    }

    /// 对齐 Java: `getPass()`。
    #[must_use]
    pub fn pass(&self) -> &str {
        self.config.pass()
    }

    /// 设置密码。
    pub fn set_pass(&mut self, pass: impl Into<String>) -> &mut Self {
        self.config.set_pass(pass);
        self
    }

    /// 对齐 Java: `getConnProps()`。
    #[must_use]
    pub fn conn_props(&self) -> &HashMap<String, String> {
        self.config.conn_props()
    }

    /// 设置连接属性。
    pub fn set_conn_props(&mut self, props: HashMap<String, String>) -> &mut Self {
        self.config.set_conn_props(props);
        self
    }

    /// 追加连接属性。
    pub fn add_conn_props(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> &mut Self {
        self.config.add_conn_props(key, value);
        self
    }

    /// 对齐 Java: `getConnection()` —— 返回包装元数据（不创建 JDBC Connection）。
    #[must_use]
    pub fn connection_meta(&self) -> DataSourceWrapper {
        self.config.to_wrapper()
    }

    /// 对齐 Java: `close()` —— 清空敏感字段。
    pub fn close(&mut self) {
        self.config.set_pass("");
    }
}

/// 对齐 Hutool `PooledDataSource`：配置 + `PoolConfig` 映射。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PooledDataSource {
    config: DbConfig,
}

impl PooledDataSource {
    /// 对齐 Java: `getDataSource`。
    #[must_use]
    pub fn get_data_source(config: DbConfig) -> Self {
        Self { config }
    }

    /// 从 URL 构造。
    #[must_use]
    pub fn new(url: impl Into<String>, user: impl Into<String>, pass: impl Into<String>) -> Self {
        Self {
            config: DbConfig::of(url, user, pass),
        }
    }

    /// 对齐 Java: `getConfig()`。
    #[must_use]
    pub fn config(&self) -> &DbConfig {
        &self.config
    }

    /// 对齐 Java: `getConnection()` —— 返回元数据包装。
    #[must_use]
    pub fn connection_meta(&self) -> DataSourceWrapper {
        self.config.to_wrapper()
    }

    /// 生成 hutool `PoolConfig`。
    #[must_use]
    pub fn pool_config(&self) -> PoolConfig {
        self.config.to_pool_config()
    }

    /// 对齐 Java: `close()`。
    pub fn close(&mut self) {
        self.config.set_pass("");
    }
}
