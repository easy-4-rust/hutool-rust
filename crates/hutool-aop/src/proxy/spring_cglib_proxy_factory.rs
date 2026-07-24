//! Proxy factories and the Hutool-aligned `ProxyUtil` facade.

use crate::{
    HandlerProxy, Method,
    aspects::Aspect,
    interceptor::{CglibInterceptor, JdkInterceptor, SpringCglibInterceptor},
};
use std::{fmt, sync::Arc};

use super::proxy::Proxy;
use super::proxy_backend::ProxyBackend;
use super::proxy_factory::ProxyFactory;

/// Factory for Spring-CGLIB-style proxies.
#[derive(Debug, Default, Clone, Copy)]
pub struct SpringCglibProxyFactory;

impl SpringCglibProxyFactory {
    /// Creates a Spring-CGLIB-style proxy.
    pub fn proxy<T, A, R, E, I>(target: T, aspect: I) -> Proxy<T, A, R, E>
    where
        I: Aspect<T, A, R, E> + 'static,
    {
        ProxyFactory::with_backend(ProxyBackend::SpringCglib).proxy(target, aspect)
    }
}
