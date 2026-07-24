//! Proxy factories and the Hutool-aligned `ProxyUtil` facade.

use crate::{
    HandlerProxy, Method,
    aspects::Aspect,
    interceptor::{CglibInterceptor, JdkInterceptor, SpringCglibInterceptor},
};
use std::{fmt, sync::Arc};

use super::proxy_factory::ProxyFactory;

/// A typed proxy produced by a [`ProxyFactory`].
pub enum Proxy<T, A, R, E> {
    /// JDK-style proxy.
    Jdk(JdkInterceptor<T, A, R, E>),
    /// CGLIB-style proxy.
    Cglib(CglibInterceptor<T, A, R, E>),
    /// Spring-CGLIB-style proxy.
    SpringCglib(SpringCglibInterceptor<T, A, R, E>),
}

impl<T: fmt::Debug, A, R, E> fmt::Debug for Proxy<T, A, R, E> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Jdk(proxy) => proxy.fmt(formatter),
            Self::Cglib(proxy) => proxy.fmt(formatter),
            Self::SpringCglib(proxy) => proxy.fmt(formatter),
        }
    }
}

impl<T, A, R, E> Proxy<T, A, R, E> {
    /// Returns the proxied target.
    #[must_use]
    pub fn get_target(&self) -> &T {
        match self {
            Self::Jdk(proxy) => proxy.get_target(),
            Self::Cglib(proxy) => proxy.get_target(),
            Self::SpringCglib(proxy) => proxy.get_target(),
        }
    }

    /// Returns the mutable proxied target.
    #[must_use]
    pub fn get_target_mut(&mut self) -> &mut T {
        match self {
            Self::Jdk(proxy) => proxy.get_target_mut(),
            Self::Cglib(proxy) => proxy.get_target_mut(),
            Self::SpringCglib(proxy) => proxy.get_target_mut(),
        }
    }

    /// Consumes the proxy and returns its target.
    #[must_use]
    pub fn into_target(self) -> T {
        match self {
            Self::Jdk(proxy) => proxy.into_target(),
            Self::Cglib(proxy) => proxy.into_target(),
            Self::SpringCglib(proxy) => proxy.into_target(),
        }
    }

    /// Invokes an operation through the selected proxy strategy.
    pub fn invoke<F>(&mut self, method: &Method, args: &mut A, operation: F) -> Result<Option<R>, E>
    where
        F: FnOnce(&mut T, &mut A) -> Result<R, E>,
    {
        match self {
            Self::Jdk(proxy) => proxy.invoke(method, args, operation),
            Self::Cglib(proxy) => proxy.invoke(method, args, operation),
            Self::SpringCglib(proxy) => proxy.invoke(method, args, operation),
        }
    }
}
