//! 对齐: `cn.hutool.http.interceptor.GlobalInterceptor`
//! 来源: hutool-http/src/main/java/cn/hutool/http/interceptor/GlobalInterceptor.java
//! 中文说明: 全局拦截器注册表，管理进程级请求和响应拦截器

use crate::config::{
    HttpInterceptorError, HttpRequestContext, HttpResponseContext, RequestInterceptor,
    ResponseInterceptor,
};
use std::sync::{Arc, Mutex, OnceLock};

fn registry() -> &'static Mutex<GlobalInterceptorState> {
    static REGISTRY: OnceLock<Mutex<GlobalInterceptorState>> = OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(GlobalInterceptorState::default()))
}

#[derive(Default)]
struct GlobalInterceptorState {
    request: Vec<RequestInterceptor>,
    response: Vec<ResponseInterceptor>,
}

/// Hutool `GlobalInterceptor` — opt-in process-scoped interceptor chains.
///
/// Java: `cn.hutool.http.GlobalInterceptor`
pub struct GlobalInterceptor;

impl GlobalInterceptor {
    /// Java: `GlobalInterceptor.addRequestInterceptor(...)`
    pub fn add_request_interceptor<F>(interceptor: F) -> &'static GlobalInterceptor
    where
        F: Fn(&mut HttpRequestContext) -> Result<(), HttpInterceptorError> + Send + Sync + 'static,
    {
        registry()
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .request
            .push(Arc::new(interceptor));
        &GlobalInterceptor
    }

    /// Java: `GlobalInterceptor.addResponseInterceptor(...)`
    pub fn add_response_interceptor<F>(interceptor: F) -> &'static GlobalInterceptor
    where
        F: Fn(&mut HttpResponseContext) -> Result<(), HttpInterceptorError> + Send + Sync + 'static,
    {
        registry()
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .response
            .push(Arc::new(interceptor));
        &GlobalInterceptor
    }

    /// Java: `GlobalInterceptor.clear()`
    pub fn clear() -> &'static GlobalInterceptor {
        let mut state = registry().lock().unwrap_or_else(|e| e.into_inner());
        state.request.clear();
        state.response.clear();
        &GlobalInterceptor
    }

    /// Java: `GlobalInterceptor.clearRequest()`
    pub fn clear_request() -> &'static GlobalInterceptor {
        registry()
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .request
            .clear();
        &GlobalInterceptor
    }

    /// Java: `GlobalInterceptor.clearResponse()`
    pub fn clear_response() -> &'static GlobalInterceptor {
        registry()
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .response
            .clear();
        &GlobalInterceptor
    }

    /// Returns a snapshot of registered request interceptors.
    #[must_use]
    pub fn request_interceptors() -> Vec<RequestInterceptor> {
        registry()
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .request
            .clone()
    }

    /// Returns a snapshot of registered response interceptors.
    #[must_use]
    pub fn response_interceptors() -> Vec<ResponseInterceptor> {
        registry()
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .response
            .clone()
    }

    /// Applies all request interceptors to `context` (fail-fast).
    pub fn apply_request(context: &mut HttpRequestContext) -> Result<(), HttpInterceptorError> {
        for interceptor in Self::request_interceptors() {
            interceptor(context)?;
        }
        Ok(())
    }

    /// Applies all response interceptors to `context` (fail-fast).
    pub fn apply_response(context: &mut HttpResponseContext) -> Result<(), HttpInterceptorError> {
        for interceptor in Self::response_interceptors() {
            interceptor(context)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::{Method, Url, header::HeaderMap};

    #[test]
    fn global_interceptor_add_clear_and_apply() {
        GlobalInterceptor::clear();
        GlobalInterceptor::add_request_interceptor(|ctx| {
            ctx.set_method(Method::HEAD);
            Ok(())
        });
        GlobalInterceptor::add_response_interceptor(|_| Ok(()));
        assert_eq!(GlobalInterceptor::request_interceptors().len(), 1);
        assert_eq!(GlobalInterceptor::response_interceptors().len(), 1);

        let mut req = HttpRequestContext::new(
            Method::GET,
            Url::parse("https://example.test/").unwrap(),
            HeaderMap::new(),
        );
        GlobalInterceptor::apply_request(&mut req).unwrap();
        assert_eq!(req.method(), &Method::HEAD);

        GlobalInterceptor::clear_request();
        assert!(GlobalInterceptor::request_interceptors().is_empty());
        GlobalInterceptor::clear_response();
        assert!(GlobalInterceptor::response_interceptors().is_empty());
        GlobalInterceptor::clear();
    }
}
