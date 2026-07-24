//! Proxy factories and the Hutool-aligned `ProxyUtil` facade.

use crate::{
    HandlerProxy, Method,
    aspects::Aspect,
    interceptor::{CglibInterceptor, JdkInterceptor, SpringCglibInterceptor},
};
use std::{fmt, sync::Arc};

mod proxy_backend;
mod proxy;
mod proxy_factory;
mod jdk_proxy_factory;
mod cglib_proxy_factory;
mod spring_cglib_proxy_factory;
mod proxy_util;

pub use proxy_backend::ProxyBackend;
pub use proxy::Proxy;
pub use proxy_factory::ProxyFactory;
pub use jdk_proxy_factory::JdkProxyFactory;
pub use cglib_proxy_factory::CglibProxyFactory;
pub use spring_cglib_proxy_factory::SpringCglibProxyFactory;
pub use proxy_util::ProxyUtil;
