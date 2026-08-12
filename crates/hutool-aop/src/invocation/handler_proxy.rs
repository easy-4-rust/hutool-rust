//! 对齐: `cn.hutool.aop.Proxy`（处理器代理部分）
//! 来源: hutool-aop/src/main/java/cn/hutool/aop/
//! 中文说明: 处理器代理，基于 InvocationHandler 的显式类型化代理实现。

use std::fmt;

use super::invocation_handler::InvocationHandler;
use super::method::Method;

/// 对齐: `cn.hutool.aop.Proxy`（处理器代理模式）
/// 中文说明: 由类型化调用处理器支撑的显式代理，组合目标对象与 InvocationHandler。
pub struct HandlerProxy<T, H> {
    target: T,
    handler: H,
}

impl<T: fmt::Debug, H> fmt::Debug for HandlerProxy<T, H> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HandlerProxy")
            .field("target", &self.target)
            .finish_non_exhaustive()
    }
}

impl<T, H> HandlerProxy<T, H> {
    /// Creates a handler-backed proxy.
    pub fn new(target: T, handler: H) -> Self {
        Self { target, handler }
    }

    /// Returns the target.
    #[must_use]
    pub fn get_target(&self) -> &T {
        &self.target
    }

    /// Returns the mutable target.
    #[must_use]
    pub fn get_target_mut(&mut self) -> &mut T {
        &mut self.target
    }

    /// Consumes the proxy and returns its target.
    #[must_use]
    pub fn into_target(self) -> T {
        self.target
    }

    /// Invokes the configured handler.
    pub fn invoke<A, R, E>(&mut self, method: &Method, args: &mut A) -> Result<R, E>
    where
        H: InvocationHandler<T, A, R, E>,
    {
        self.handler.invoke(&mut self.target, method, args)
    }
}
