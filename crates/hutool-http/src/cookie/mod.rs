//! 对齐: `cn.hutool.http.cookie`
//! 来源: hutool-http/src/main/java/cn/hutool/http/cookie/
//! 中文说明: Cookie存储管理，线程本地Cookie jar实现，支持全局和线程级Cookie管理

use crate::HttpCookie;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

thread_local! {
    static THREAD_STORE: RefCell<CookieJar> = RefCell::new(CookieJar::default());
}

/// In-memory cookie jar keyed by URI string.
#[derive(Debug, Clone, Default)]
pub struct CookieJar {
    /// URI → cookies for that URI.
    by_uri: HashMap<String, Vec<HttpCookie>>,
}

impl CookieJar {
    /// Adds a cookie for `uri`.
    pub fn add(&mut self, uri: impl Into<String>, cookie: HttpCookie) {
        self.by_uri.entry(uri.into()).or_default().push(cookie);
    }

    /// Returns cookies for `uri` (empty when none).
    #[must_use]
    pub fn get(&self, uri: &str) -> Vec<HttpCookie> {
        self.by_uri.get(uri).cloned().unwrap_or_default()
    }

    /// Returns every cookie across all URIs.
    #[must_use]
    pub fn get_cookies(&self) -> Vec<HttpCookie> {
        self.by_uri.values().flat_map(|v| v.iter().cloned()).collect()
    }

    /// Returns all stored URI keys.
    #[must_use]
    pub fn get_uris(&self) -> Vec<String> {
        self.by_uri.keys().cloned().collect()
    }

    /// Removes a matching cookie for `uri`; returns whether anything was removed.
    pub fn remove(&mut self, uri: &str, cookie: &HttpCookie) -> bool {
        let Some(list) = self.by_uri.get_mut(uri) else {
            return false;
        };
        let before = list.len();
        list.retain(|c| c.name() != cookie.name() || c.value() != cookie.value());
        let changed = list.len() != before;
        if list.is_empty() {
            self.by_uri.remove(uri);
        }
        changed
    }

    /// Clears all cookies; returns whether the jar was non-empty.
    pub fn remove_all(&mut self) -> bool {
        let had = !self.by_uri.is_empty();
        self.by_uri.clear();
        had
    }
}

/// Shared cookie-manager handle (Hutool `CookieManager` stand-in).
pub type CookieManagerHandle = Arc<Mutex<CookieJar>>;

fn global_manager() -> &'static Mutex<Option<CookieManagerHandle>> {
    static MANAGER: OnceLock<Mutex<Option<CookieManagerHandle>>> = OnceLock::new();
    MANAGER.get_or_init(|| Mutex::new(None))
}

/// Hutool `GlobalCookieManager` — opt-in shared cookie manager handle.
///
/// Java: `cn.hutool.http.cookie.GlobalCookieManager`
pub struct GlobalCookieManager;

impl GlobalCookieManager {
    /// Java: `GlobalCookieManager.setCookieManager(CookieManager)`
    pub fn set_cookie_manager(manager: CookieManagerHandle) {
        *global_manager().lock().unwrap_or_else(|e| e.into_inner()) = Some(manager);
    }

    /// Java: `GlobalCookieManager.getCookieManager()`
    #[must_use]
    pub fn get_cookie_manager() -> CookieManagerHandle {
        let mut guard = global_manager().lock().unwrap_or_else(|e| e.into_inner());
        if guard.is_none() {
            *guard = Some(Arc::new(Mutex::new(CookieJar::default())));
        }
        Arc::clone(guard.as_ref().expect("cookie manager initialized"))
    }

    /// Java: `GlobalCookieManager.closeCookie()` — drops the shared manager.
    pub fn close_cookie() {
        *global_manager().lock().unwrap_or_else(|e| e.into_inner()) = None;
        ThreadLocalCookieStore::remove_current();
    }

    /// Returns cookies for a request URL from the shared manager.
    ///
    /// Java: `GlobalCookieManager.getCookies(HttpConnection)` — Rust takes a URL string
    /// instead of `HttpConnection`.
    #[must_use]
    pub fn get_cookies(url: &str) -> Vec<HttpCookie> {
        let manager = Self::get_cookie_manager();
        manager
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .get(url)
    }

    /// Adds cookies parsed from `Set-Cookie` style `name=value` pairs for `url`.
    ///
    /// Java: `GlobalCookieManager.add(HttpConnection)` / `store(HttpConnection)`
    pub fn add(url: &str, cookies: &[HttpCookie]) {
        let manager = Self::get_cookie_manager();
        let mut jar = manager.lock().unwrap_or_else(|e| e.into_inner());
        for cookie in cookies {
            jar.add(url, cookie.clone());
        }
    }

    /// Alias of [`Self::add`] matching Hutool `store`.
    ///
    /// Java: `GlobalCookieManager.store(HttpConnection)`
    pub fn store(url: &str, cookies: &[HttpCookie]) {
        Self::add(url, cookies);
    }
}

/// Thread-local cookie store aligned with Hutool `ThreadLocalCookieStore`.
///
/// Java: `cn.hutool.http.cookie.ThreadLocalCookieStore`
pub struct ThreadLocalCookieStore;

impl ThreadLocalCookieStore {
    /// Returns a snapshot of the current thread's jar.
    ///
    /// Java: `ThreadLocalCookieStore.getCookieStore()`
    #[must_use]
    pub fn get_cookie_store() -> CookieJar {
        THREAD_STORE.with(|cell| cell.borrow().clone())
    }

    /// Clears the current thread's jar.
    ///
    /// Java: `ThreadLocalCookieStore.removeCurrent()`
    pub fn remove_current() {
        THREAD_STORE.with(|cell| {
            cell.borrow_mut().remove_all();
        });
    }

    /// Java: `ThreadLocalCookieStore.add(URI, HttpCookie)`
    pub fn add(uri: impl Into<String>, cookie: HttpCookie) {
        THREAD_STORE.with(|cell| cell.borrow_mut().add(uri, cookie));
    }

    /// Java: `ThreadLocalCookieStore.get(URI)`
    #[must_use]
    pub fn get(uri: &str) -> Vec<HttpCookie> {
        THREAD_STORE.with(|cell| cell.borrow().get(uri))
    }

    /// Java: `ThreadLocalCookieStore.getCookies()`
    #[must_use]
    pub fn get_cookies() -> Vec<HttpCookie> {
        THREAD_STORE.with(|cell| cell.borrow().get_cookies())
    }

    /// Java: `ThreadLocalCookieStore.getURIs()`
    #[must_use]
    pub fn get_uris() -> Vec<String> {
        THREAD_STORE.with(|cell| cell.borrow().get_uris())
    }

    /// Java: `ThreadLocalCookieStore.remove(URI, HttpCookie)`
    pub fn remove(uri: &str, cookie: &HttpCookie) -> bool {
        THREAD_STORE.with(|cell| cell.borrow_mut().remove(uri, cookie))
    }

    /// Java: `ThreadLocalCookieStore.removeAll()`
    pub fn remove_all() -> bool {
        THREAD_STORE.with(|cell| cell.borrow_mut().remove_all())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thread_local_cookie_store_roundtrip() {
        ThreadLocalCookieStore::remove_current();
        ThreadLocalCookieStore::add("https://a.test/", HttpCookie::new("a", "1"));
        ThreadLocalCookieStore::add("https://a.test/", HttpCookie::new("b", "2"));
        assert_eq!(ThreadLocalCookieStore::get("https://a.test/").len(), 2);
        assert_eq!(ThreadLocalCookieStore::get_cookies().len(), 2);
        assert!(ThreadLocalCookieStore::get_uris().contains(&"https://a.test/".to_string()));
        assert!(ThreadLocalCookieStore::remove(
            "https://a.test/",
            &HttpCookie::new("a", "1")
        ));
        assert!(ThreadLocalCookieStore::remove_all());
        assert!(ThreadLocalCookieStore::get_cookies().is_empty());
    }

    #[test]
    fn global_cookie_manager_store_and_close() {
        GlobalCookieManager::close_cookie();
        GlobalCookieManager::store(
            "https://b.test/",
            &[HttpCookie::new("sid", "xyz")],
        );
        let cookies = GlobalCookieManager::get_cookies("https://b.test/");
        assert_eq!(cookies.len(), 1);
        assert_eq!(cookies[0].name(), "sid");
        GlobalCookieManager::close_cookie();
        // After close, a fresh manager is lazily created empty.
        assert!(GlobalCookieManager::get_cookies("https://b.test/").is_empty());
        GlobalCookieManager::close_cookie();
    }
}
