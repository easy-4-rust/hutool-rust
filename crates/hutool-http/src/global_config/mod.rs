//! Process-scoped HTTP defaults aligned with Hutool `HttpGlobalConfig`.
//!
//! Values are stored for Hutool callers that opt in; [`HttpRequest`] /
//! [`HttpClient`] do **not** auto-apply them unless the caller reads and
//! overlays them (avoids silent process-global timeout injection).

use crate::cookie::{CookieManagerHandle, GlobalCookieManager};
use crate::HostnameVerification;
use std::sync::{Mutex, OnceLock};
use std::time::Duration;

mod http_global_config_state;
mod http_global_config;

pub use http_global_config_state::HttpGlobalConfigState;
pub use http_global_config::HttpGlobalConfig;

pub const DEFAULT_BOUNDARY: &str = "----HiToolHttpBoundary";

fn state() -> &'static Mutex<HttpGlobalConfigState> {
    static STATE: OnceLock<Mutex<HttpGlobalConfigState>> = OnceLock::new();
    STATE.get_or_init(|| Mutex::new(HttpGlobalConfigState::default()))
}
