//! 对齐: `cn.hutool.aop.proxy.CglibProxyFactory`
//! 来源: hutool-aop/src/main/java/cn/hutool/aop/proxy/CglibProxyFactory.java
//! 中文说明: CGLIB 风格代理工厂，使用 CGLIB 回调顺序创建代理。

use crate::aspects::Aspect;

use super::proxy::Proxy;
use super::proxy_backend::ProxyBackend;
use super::proxy_factory::ProxyFactory;

/// 对齐: `cn.hutool.aop.proxy.CglibProxyFactory`
/// 中文说明: CGLIB 风格代理工厂，通过 CGLIB 回调顺序创建切面代理。
#[derive(Debug, Default, Clone, Copy)]
pub struct CglibProxyFactory;

impl CglibProxyFactory {
    /// Creates a CGLIB-style proxy.
    pub fn proxy<T, A, R, E, I>(target: T, aspect: I) -> Proxy<T, A, R, E>
    where
        I: Aspect<T, A, R, E> + 'static,
    {
        ProxyFactory::with_backend(ProxyBackend::Cglib).proxy(target, aspect)
    }
}
