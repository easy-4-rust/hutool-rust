//! 对齐: `cn.hutool.http` (全局配置模块)
//! 来源: hutool-http/src/main/java/cn/hutool/http/HttpGlobalConfig.java
//! 中文说明: 全局HTTP配置模块，管理进程级HTTP默认超时和Cookie设置

use std::sync::{Mutex, OnceLock};

mod http_global_config_state;
mod http_global_config;

pub use http_global_config_state::HttpGlobalConfigState;
pub use http_global_config::HttpGlobalConfig;

/// 默认 multipart 边界字符串，对齐 Hutool `HttpGlobalConfig.DEFAULT_BOUNDARY`。
pub const DEFAULT_BOUNDARY: &str = "----HiToolHttpBoundary";

fn state() -> &'static Mutex<HttpGlobalConfigState> {
    static STATE: OnceLock<Mutex<HttpGlobalConfigState>> = OnceLock::new();
    STATE.get_or_init(|| Mutex::new(HttpGlobalConfigState::default()))
}
