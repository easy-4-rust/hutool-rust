//! GlobalDbConfig facade，对齐 hutool 的 `cn.hutool.db.GlobalDbConfig`。
//!
//! 全局数据库配置（大小写敏感、是否返回生成键、是否显示 SQL 等）。

mod global_db_config;
mod log_level;

pub use global_db_config::GlobalDbConfig;
pub use log_level::LogLevel;
