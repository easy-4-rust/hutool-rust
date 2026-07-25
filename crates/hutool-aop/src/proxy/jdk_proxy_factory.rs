//! 对齐: `cn.hutool.aop.proxy.JdkProxyFactory`
//! 来源: hutool-aop/src/main/java/cn/hutool/aop/proxy/JdkProxyFactory.java
//! 中文说明: JDK 风格代理工厂，使用 JDK 回调顺序创建代理。

use crate::{
    HandlerProxy, Method,
    aspects::Aspect,
    interceptor::{CglibInterceptor, JdkInterceptor, SpringCglibInterceptor},
};
use std::{fmt, sync::Arc};

use super::proxy::Proxy;
use super::proxy_backend::ProxyBackend;
use super::proxy_factory::ProxyFactory;

/// 对齐: `cn.hutool.aop.proxy.JdkProxyFactory`
/// 中文说明: JDK 风格代理工厂，通过 JDK 回调顺序创建切面代理。
#[derive(Debug, Default, Clone, Copy)]
pub struct JdkProxyFactory;

impl JdkProxyFactory {
    /// Creates a JDK-style proxy.
    pub fn proxy<T, A, R, E, I>(target: T, aspect: I) -> Proxy<T, A, R, E>
    where
        I: Aspect<T, A, R, E> + 'static,
    {
        ProxyFactory::with_backend(ProxyBackend::Jdk).proxy(target, aspect)
    }
}
