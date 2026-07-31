//! 对齐: `cn.hutool.http.HttpGlobalConfig`
//! 来源: hutool-http/src/main/java/cn/hutool/http/HttpGlobalConfig.java
//! 中文说明: 全局HTTP配置静态访问器，提供进程级HTTP默认设置

use crate::cookie::{CookieManagerHandle, GlobalCookieManager};
use crate::HostnameVerification;
use std::time::Duration;

use super::http_global_config_state::HttpGlobalConfigState;

/// Static accessors matching Hutool `HttpGlobalConfig`.
pub struct HttpGlobalConfig;

impl HttpGlobalConfig {
    /// Returns a snapshot of the current global settings.
    #[must_use]
    pub fn snapshot() -> HttpGlobalConfigState {
        state()
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clone()
    }

    /// Resets all global settings to defaults (tests).
    pub fn reset() {
        *state().lock().unwrap_or_else(|e| e.into_inner()) = HttpGlobalConfigState::default();
        GlobalCookieManager::close_cookie();
    }

    /// Java: `HttpGlobalConfig.getTimeout()`
    #[must_use]
    pub fn get_timeout() -> i32 {
        Self::snapshot().timeout_ms
    }

    /// Java: `HttpGlobalConfig.setTimeout(int)`
    pub fn set_timeout(timeout_ms: i32) {
        state().lock().unwrap_or_else(|e| e.into_inner()).timeout_ms = timeout_ms;
    }

    /// Timeout as [`Duration`] when a non-negative millisecond value is set.
    #[must_use]
    pub fn timeout_duration() -> Option<Duration> {
        let ms = Self::get_timeout();
        if ms < 0 {
            None
        } else {
            Some(Duration::from_millis(ms as u64))
        }
    }

    /// Java: `HttpGlobalConfig.getBoundary()`
    #[must_use]
    pub fn get_boundary() -> String {
        Self::snapshot().boundary
    }

    /// Java: `HttpGlobalConfig.setBoundary(String)`
    pub fn set_boundary(boundary: impl Into<String>) {
        state().lock().unwrap_or_else(|e| e.into_inner()).boundary = boundary.into();
    }

    /// Java: `HttpGlobalConfig.getMaxRedirectCount()`
    #[must_use]
    pub fn get_max_redirect_count() -> i32 {
        Self::snapshot().max_redirect_count
    }

    /// Java: `HttpGlobalConfig.setMaxRedirectCount(int)`
    pub fn set_max_redirect_count(count: i32) {
        state()
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .max_redirect_count = count;
    }

    /// Java: `HttpGlobalConfig.isIgnoreEOFError()`
    #[must_use]
    pub fn is_ignore_eof_error() -> bool {
        Self::snapshot().ignore_eof_error
    }

    /// Java: `HttpGlobalConfig.setIgnoreEOFError(boolean)`
    pub fn set_ignore_eof_error(ignore: bool) {
        state()
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .ignore_eof_error = ignore;
    }

    /// Java: `HttpGlobalConfig.isDecodeUrl()`
    #[must_use]
    pub fn is_decode_url() -> bool {
        Self::snapshot().decode_url
    }

    /// Java: `HttpGlobalConfig.setDecodeUrl(boolean)`
    pub fn set_decode_url(decode: bool) {
        state().lock().unwrap_or_else(|e| e.into_inner()).decode_url = decode;
    }

    /// Java: `HttpGlobalConfig.getCookieManager()` — returns the shared handle.
    #[must_use]
    pub fn get_cookie_manager() -> CookieManagerHandle {
        GlobalCookieManager::get_cookie_manager()
    }

    /// Java: `HttpGlobalConfig.setCookieManager(CookieManager)`
    pub fn set_cookie_manager(manager: CookieManagerHandle) {
        GlobalCookieManager::set_cookie_manager(manager);
    }

    /// Java: `HttpGlobalConfig.closeCookie()`
    pub fn close_cookie() {
        GlobalCookieManager::close_cookie();
    }

    /// Java: `HttpGlobalConfig.allowPatch()` — PATCH is always available via reqwest.
    pub fn allow_patch() {
        state().lock().unwrap_or_else(|e| e.into_inner()).allow_patch = true;
    }

    /// Java: `HttpGlobalConfig.isTrustAnyHost()`
    #[must_use]
    pub fn is_trust_any_host() -> bool {
        Self::snapshot().trust_any_host
    }

    /// Java: `HttpGlobalConfig.setTrustAnyHost(boolean)`
    pub fn set_trust_any_host(trust: bool) {
        state()
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .trust_any_host = trust;
    }

    /// Maps [`Self::is_trust_any_host`] to a hostname verification mode.
    #[must_use]
    pub fn hostname_verification() -> HostnameVerification {
        if Self::is_trust_any_host() {
            HostnameVerification::DangerousAcceptInvalid
        } else {
            HostnameVerification::Strict
        }
    }
}

use super::state;
