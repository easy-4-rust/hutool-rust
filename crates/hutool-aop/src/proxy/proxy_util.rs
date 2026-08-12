//! 对齐: `cn.hutool.aop.proxy.ProxyUtil`
//! 来源: hutool-aop/src/main/java/cn/hutool/aop/proxy/ProxyUtil.java
//! 中文说明: 代理工具类，提供创建代理实例的便捷静态方法。

use crate::{HandlerProxy, aspects::Aspect};

use super::proxy::Proxy;
use super::proxy_factory::ProxyFactory;

/// 对齐: `cn.hutool.aop.proxy.ProxyUtil`
/// 中文说明: Hutool 对齐的代理工具门面，提供创建切面代理和处理器代理的便捷方法。
#[derive(Debug, Default, Clone, Copy)]
pub struct ProxyUtil;

impl ProxyUtil {
    /// Creates a default JDK-style aspect proxy.
    pub fn proxy<T, A, R, E, I>(target: T, aspect: I) -> Proxy<T, A, R, E>
    where
        I: Aspect<T, A, R, E> + 'static,
    {
        ProxyFactory::create_proxy(target, aspect)
    }

    /// Creates a proxy with a default-constructed aspect type.
    pub fn proxy_default<T, A, R, E, I>(target: T) -> Proxy<T, A, R, E>
    where
        I: Aspect<T, A, R, E> + Default + 'static,
    {
        ProxyFactory::create_proxy_default::<T, A, R, E, I>(target)
    }

    /// Creates an explicit typed invocation-handler proxy.
    pub fn new_proxy_instance<T, H>(target: T, handler: H) -> HandlerProxy<T, H> {
        HandlerProxy::new(target, handler)
    }

    /// Creates a handler proxy while accepting an explicit loader marker.
    ///
    /// Rust does not expose JVM class loaders; the marker lets adapters retain
    /// ownership/lifetime context without affecting dispatch.
    pub fn new_proxy_instance_with_loader<T, H, L>(
        target: T,
        handler: H,
        _loader: &L,
    ) -> HandlerProxy<T, H> {
        HandlerProxy::new(target, handler)
    }
}
