//! NoSQL 配置桩 —— 对齐 Hutool `cn.hutool.db.nosql.*` 测试期望。

/// Redis 数据源配置。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedisDs {
    host: String,
}

impl RedisDs {
    /// 对齐 Java: `RedisDS.create()`.
    #[must_use]
    pub fn create() -> Self {
        Self {
            host: "localhost".to_string(),
        }
    }

    /// 返回主机名（桩实现）。
    #[must_use]
    pub fn host(&self) -> &str {
        &self.host
    }
}
