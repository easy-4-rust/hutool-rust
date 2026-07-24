//! NoSQL 配置桩 —— 对齐 Hutool `cn.hutool.db.nosql.*` 测试期望。

mod mongo_ds;
mod redis_ds;

pub use mongo_ds::MongoDs;
pub use redis_ds::RedisDs;
