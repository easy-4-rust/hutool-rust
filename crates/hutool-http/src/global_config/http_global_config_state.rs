//! Process-scoped HTTP defaults aligned with Hutool `HttpGlobalConfig`.
//!
//! Values are stored for Hutool callers that opt in; [`HttpRequest`] /
//! [`HttpClient`] do **not** auto-apply them unless the caller reads and
//! overlays them (avoids silent process-global timeout injection).

use crate::cookie::{CookieManagerHandle, GlobalCookieManager};
use crate::HostnameVerification;
use std::sync::{Mutex, OnceLock};
use std::time::Duration;

use super::http_global_config::HttpGlobalConfig;

/// Hutool-aligned global HTTP settings store.
///
/// Java: `cn.hutool.http.HttpGlobalConfig`
#[derive(Debug, Clone)]
pub struct HttpGlobalConfigState {
    /// Connect/read timeout in milliseconds (`-1` = unset / infinite in Hutool).
    pub timeout_ms: i32,
    /// Multipart boundary string.
    pub boundary: String,
    /// Maximum redirect hops.
    pub max_redirect_count: i32,
    /// Whether to ignore EOF / truncated body errors.
    pub ignore_eof_error: bool,
    /// Whether to decode URLs before sending.
    pub decode_url: bool,
    /// Whether PATCH is allowed (always true for reqwest; flag retained for parity).
    pub allow_patch: bool,
    /// Whether to trust any host (Dangerous hostname verification when applied).
    pub trust_any_host: bool,
}

impl Default for HttpGlobalConfigState {
    fn default() -> Self {
        Self {
            timeout_ms: -1,
            boundary: DEFAULT_BOUNDARY.to_string(),
            max_redirect_count: 0,
            ignore_eof_error: true,
            decode_url: false,
            allow_patch: true,
            trust_any_host: false,
        }
    }
}

use super::{DEFAULT_BOUNDARY, state};
