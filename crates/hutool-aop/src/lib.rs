//! 对齐: `cn.hutool.aop`
//! 来源: hutool-aop/src/main/java/cn/hutool/aop/
//! 中文说明: Hutool AOP 模块，提供面向切面编程的代理、拦截器和切面支持。
//!
//! Typed interception aligned with Hutool's AOP capability.
//!
//! Rust has no JVM-style runtime class proxy. `HiTool` therefore preserves
//! Hutool's callback and suppression semantics through explicit, type-safe
//! proxy wrappers, while retaining a composable interceptor chain.

#![forbid(unsafe_code)]

mod invocation;

pub mod aspects;
pub mod interceptor;
pub mod prelude;
pub mod proxy;

pub use invocation::{HandlerProxy, InvocationHandler, Method};
pub use proxy::{Proxy, ProxyBackend, ProxyFactory, ProxyUtil};

use std::{fmt, marker::PhantomData, sync::Arc};

#[cfg(test)]
mod aop_tests;

/// 对齐: `cn.hutool.aop.interceptor` 包中的拦截器概念
/// 中文说明: 同步环绕拦截器，对目标操作进行前置和后置增强处理。
pub trait Interceptor<C, R, E>: Send + Sync {
    /// Invokes behavior around the rest of the chain.
    fn intercept(
        &self,
        context: &mut C,
        next: &mut dyn FnMut(&mut C) -> Result<R, E>,
    ) -> Result<R, E>;
}

/// 对齐: Hutool 拦截器链模式
/// 中文说明: 有序、不可变的拦截器链，按顺序执行所有拦截器后再执行目标操作。
pub struct InterceptorChain<C, R, E> {
    interceptors: Vec<Arc<dyn Interceptor<C, R, E>>>,
}

impl<C, R, E> fmt::Debug for InterceptorChain<C, R, E> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("InterceptorChain")
            .field("interceptor_count", &self.interceptors.len())
            .finish()
    }
}

impl<C, R, E> Default for InterceptorChain<C, R, E> {
    fn default() -> Self {
        Self {
            interceptors: Vec::new(),
        }
    }
}

impl<C, R, E> InterceptorChain<C, R, E> {
    /// Creates an empty chain.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Appends an interceptor.
    #[must_use]
    pub fn with<I>(mut self, interceptor: I) -> Self
    where
        I: Interceptor<C, R, E> + 'static,
    {
        self.interceptors.push(Arc::new(interceptor));
        self
    }

    /// Appends a shared interceptor.
    #[must_use]
    pub fn with_shared(mut self, interceptor: Arc<dyn Interceptor<C, R, E>>) -> Self {
        self.interceptors.push(interceptor);
        self
    }

    /// Executes the chain followed by the target operation.
    pub fn execute<F>(&self, context: &mut C, mut target: F) -> Result<R, E>
    where
        F: FnMut(&mut C) -> Result<R, E>,
    {
        dispatch(&self.interceptors, context, &mut target)
    }

    /// Returns the number of configured interceptors.
    #[must_use]
    pub fn len(&self) -> usize {
        self.interceptors.len()
    }

    /// Returns whether the chain has no interceptors.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.interceptors.is_empty()
    }
}

fn dispatch<C, R, E>(
    interceptors: &[Arc<dyn Interceptor<C, R, E>>],
    context: &mut C,
    target: &mut dyn FnMut(&mut C) -> Result<R, E>,
) -> Result<R, E> {
    let Some((current, remaining)) = interceptors.split_first() else {
        return target(context);
    };
    current.intercept(context, &mut |context| dispatch(remaining, context, target))
}

/// 对齐: Hutool 切面前后置回调模式
/// 中文说明: 基于前置和后置回调的拦截器实现。
pub struct BeforeAfter<C, R, E, B, A> {
    before: B,
    after: A,
    marker: PhantomData<fn(C, R, E)>,
}

impl<C, R, E, B, A> BeforeAfter<C, R, E, B, A> {
    /// Creates a callback-based interceptor.
    pub fn new(before: B, after: A) -> Self {
        Self {
            before,
            after,
            marker: PhantomData,
        }
    }
}

impl<C, R, E, B, A> Interceptor<C, R, E> for BeforeAfter<C, R, E, B, A>
where
    B: Fn(&mut C) + Send + Sync,
    A: Fn(&mut C, &Result<R, E>) + Send + Sync,
{
    fn intercept(
        &self,
        context: &mut C,
        next: &mut dyn FnMut(&mut C) -> Result<R, E>,
    ) -> Result<R, E> {
        (self.before)(context);
        let result = next(context);
        (self.after)(context, &result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interceptors_wrap_target_in_order() {
        let chain = InterceptorChain::<Vec<String>, usize, ()>::new()
            .with(BeforeAfter::new(
                |events: &mut Vec<String>| events.push("before-1".into()),
                |events: &mut Vec<String>, _: &Result<usize, ()>| events.push("after-1".into()),
            ))
            .with(BeforeAfter::new(
                |events: &mut Vec<String>| events.push("before-2".into()),
                |events: &mut Vec<String>, _: &Result<usize, ()>| events.push("after-2".into()),
            ));
        let mut events = Vec::new();
        let count = chain
            .execute(&mut events, |events| {
                events.push("target".into());
                Ok(events.len())
            })
            .unwrap();
        assert_eq!(count, 3);
        assert_eq!(
            events,
            ["before-1", "before-2", "target", "after-2", "after-1"]
        );
    }
}
