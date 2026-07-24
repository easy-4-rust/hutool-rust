//! GlobalDbConfig facade，对齐 hutool 的 `cn.hutool.db.GlobalDbConfig`。
//!
//! 全局数据库配置（大小写敏感、是否返回生成键、是否显示 SQL 等）。

use std::sync::Mutex;

mod log_level;
mod global_db_config;

pub use log_level::LogLevel;
pub use global_db_config::GlobalDbConfig;
