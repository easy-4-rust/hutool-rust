//! 对齐: `cn.hutool.aop.proxy`
//! 来源: hutool-aop/src/main/java/cn/hutool/aop/proxy/
//! 中文说明: 代理工厂模块，提供 JDK/CGLIB/SpringCglib 多种代理策略及 ProxyUtil 便捷门面。

mod cglib_proxy_factory;
mod jdk_proxy_factory;
mod proxy;
mod proxy_backend;
mod proxy_factory;
mod proxy_util;
mod spring_cglib_proxy_factory;

pub use cglib_proxy_factory::CglibProxyFactory;
pub use jdk_proxy_factory::JdkProxyFactory;
pub use proxy::Proxy;
pub use proxy_backend::ProxyBackend;
pub use proxy_factory::ProxyFactory;
pub use proxy_util::ProxyUtil;
pub use spring_cglib_proxy_factory::SpringCglibProxyFactory;
