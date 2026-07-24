//! Proxy factories and the Hutool-aligned `ProxyUtil` facade.

use crate::{
    HandlerProxy, Method,
    aspects::Aspect,
    interceptor::{CglibInterceptor, JdkInterceptor, SpringCglibInterceptor},
};
use std::{fmt, sync::Arc};

use super::proxy::Proxy;
use super::proxy_backend::ProxyBackend;

/// Selects and constructs typed aspect proxies.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ProxyFactory {
    backend: ProxyBackend,
}

impl ProxyFactory {
    /// Creates the default JDK-style factory.
    #[must_use]
    pub fn create() -> Self {
        Self::default()
    }

    /// Creates a factory for an explicit backend.
    #[must_use]
    pub const fn with_backend(backend: ProxyBackend) -> Self {
        Self { backend }
    }

    /// Returns the selected backend.
    #[must_use]
    pub const fn backend(self) -> ProxyBackend {
        self.backend
    }

    /// Creates a proxy with an owned aspect.
    pub fn proxy<T, A, R, E, I>(self, target: T, aspect: I) -> Proxy<T, A, R, E>
    where
        I: Aspect<T, A, R, E> + 'static,
    {
        self.proxy_shared(target, Arc::new(aspect))
    }

    /// Creates a proxy by default-constructing its aspect type.
    pub fn proxy_default<T, A, R, E, I>(self, target: T) -> Proxy<T, A, R, E>
    where
        I: Aspect<T, A, R, E> + Default + 'static,
    {
        self.proxy(target, I::default())
    }

    /// Creates a proxy with a shared aspect.
    pub fn proxy_shared<T, A, R, E>(
        self,
        target: T,
        aspect: Arc<dyn Aspect<T, A, R, E>>,
    ) -> Proxy<T, A, R, E> {
        match self.backend {
            ProxyBackend::Jdk => Proxy::Jdk(JdkInterceptor::with_shared(target, aspect)),
            ProxyBackend::Cglib => Proxy::Cglib(CglibInterceptor::with_shared(target, aspect)),
            ProxyBackend::SpringCglib => {
                Proxy::SpringCglib(SpringCglibInterceptor::with_shared(target, aspect))
            }
        }
    }

    /// Creates a default JDK-style proxy.
    pub fn create_proxy<T, A, R, E, I>(target: T, aspect: I) -> Proxy<T, A, R, E>
    where
        I: Aspect<T, A, R, E> + 'static,
    {
        Self::create().proxy(target, aspect)
    }

    /// Creates a default proxy with a default-constructed aspect.
    pub fn create_proxy_default<T, A, R, E, I>(target: T) -> Proxy<T, A, R, E>
    where
        I: Aspect<T, A, R, E> + Default + 'static,
    {
        Self::create().proxy_default::<T, A, R, E, I>(target)
    }
}
